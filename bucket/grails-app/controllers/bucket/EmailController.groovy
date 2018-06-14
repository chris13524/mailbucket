package bucket

import javax.mail.BodyPart
import javax.mail.Header
import javax.mail.Session
import javax.mail.internet.MimeMessage
import javax.mail.internet.MimeMultipart

class EmailController {
	EmailService emailService
	
	def retrieve() {
		String emailAddress = params.emailAddress
		if (emailAddress == null) {
			response.sendError(400)
			return
		}
		
		Email email = emailService.popLatestEmail(emailAddress)
		
		if (email == null) {
			response.sendError(404)
			return []
		}
		
		String to = null
		String from = null
		String subject = null
		Map<String, String> headers = [:]
		List<Map<String, String>> rawHeaders = []
		
		String contentType = null
		String rawContentType = null
		def body = null
		def rawBody = null
		String text = null
		String html = null
		
		Session s = Session.getInstance(new Properties())
		MimeMessage message = new MimeMessage(s, new ByteArrayInputStream(email.payload.getBytes()))
		
		(headers, rawHeaders) = renderHeaders(message.getAllHeaders())
		to = headers["To"]
		from = headers["From"]
		subject = headers["Subject"]
		
		rawContentType = message.getContentType()
		contentType = rawContentType.split(";")[0]
		
		// get the content of the email
		Object content = message.getContent()
		
		// convert it into a map tree
		(body, rawBody) = renderContent(content)
		
		// if multipart, extract the plain text and html parts
		if (body instanceof Map) {
			Map<String, Object> parts = (Map) body
			text = parts["text/plain"]
			html = parts["text/html"]
		} else if (body instanceof String) {
			if (contentType == "text/plain") {
				text = body
			} else if (contentType == "text/html") {
				html = body
			}
		}
		
		response.status = 200
		return [
				smtpTo        : email.smtpTo,
				smtpFrom      : email.smtpFrom,
				raw           : email.payload,
				
				to            : to,
				from          : from,
				subject       : subject,
				
				headers       : headers,
				rawHeaders    : rawHeaders,
				
				contentType   : contentType,
				rawContentType: rawContentType,
				body          : body,
				rawBody       : rawBody,
				text          : text,
				html          : html,
		]
	}
	
	private static Tuple2<Object, Object> renderContent(Object content) {
		if (content == null) {
			return new Tuple2(null, null)
		} else if (content instanceof String) {
			return new Tuple2(content, content)
		} else if (content instanceof MimeMultipart) {
			MimeMultipart multipart = (MimeMultipart) content
			Map<String, Object> result = [:]
			List<Object> rawResult = []
			for (int i = 0; i < multipart.count; i++) {
				BodyPart part = multipart.getBodyPart(i)
				
				String rawType = part.contentType
				String type = rawType.split(";")[0]
				Object body = renderContent(part.content).first
				
				Map<String, String> headers
				List<Map<String, String>> rawHeaders
				(headers, rawHeaders) = renderHeaders(part.allHeaders)
				
				result[type] = body
				rawResult << [
						type      : type,
						rawType   : rawType,
						headers   : headers,
						rawHeaders: rawHeaders,
						body      : body,
				]
			}
			return new Tuple2(result, rawResult)
		} else if (content instanceof ByteArrayInputStream) {
			String res = Utils.convertStreamToString(content)
			return new Tuple2(res, res)
		} else {
			String res = content.toString()
			return new Tuple2(res, res)
		}
	}
	
	private
	static Tuple2<Map<String, String>, List<Map<String, String>>> renderHeaders(Enumeration<Header> headerEnumeration) {
		Map<String, String> headers = [:]
		List<Map<String, String>> rawHeaders = []
		
		for (Enumeration<Header> e = headerEnumeration; e.hasMoreElements();) {
			Header h = e.nextElement()
			headers[h.name] = h.value
			rawHeaders << [
					name : h.name,
					value: h.value,
			]
		}
		
		return new Tuple2(headers, rawHeaders)
	}
}