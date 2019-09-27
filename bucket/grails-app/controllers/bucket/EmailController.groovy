package bucket

import grails.converters.JSON

import javax.mail.BodyPart
import javax.mail.Header
import javax.mail.Session
import javax.mail.internet.MimeMessage
import javax.mail.internet.MimeMultipart

class EmailController {
	EmailService emailService
	
	def retrieve() {
		log.trace("retrieve()")
		String emailAddress = params.emailAddress
		if (emailAddress == null) {
			response.sendError(400)
			return
		}
		log.trace("emailAddress: $emailAddress")
		
		Email email = emailService.popLatestEmail(emailAddress)
		log.trace("email: ${email as JSON}")
		
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
		
		if (body instanceof Map) {
			(text, html) = extractTextHtml(body)
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
				Tuple2<Object, Object> tmpBody = renderContent(part.content)
				Object body = tmpBody.first
				Object rawBody = tmpBody.second
				
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
						rawBody   : rawBody
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
	
	private static Tuple2<String, String> extractTextHtml(Map<String, Object> body) {
		String text = null
		String html = null
		
		for (Map.Entry<String, Object> part : body) {
			if (part.key == "text/plain") {
				text = part.value
			} else if (part.key == "text/html") {
				html = part.value
			} else if (part.key.startsWith("multipart/")) {
				def (String textCandidate, String htmlCandidate) = extractTextHtml(part.value as Map)
				if (text == null) text = textCandidate
				if (html == null) html = htmlCandidate
			}
		}
		
		return new Tuple2<String, String>(text, html)
	}
}
