package bucket

import grails.compiler.GrailsCompileStatic
import org.subethamail.smtp.MessageContext
import org.subethamail.smtp.MessageHandler
import org.subethamail.smtp.MessageHandlerFactory
import org.subethamail.smtp.RejectException

import javax.mail.Header as MHeader
import javax.mail.Session
import javax.mail.internet.MimeMessage
import java.util.function.Consumer

@GrailsCompileStatic
class MyMessageHandlerFactory implements MessageHandlerFactory {
	final Consumer<Email> emailHandler
	
	MyMessageHandlerFactory(Consumer<Email> emailHandler) {
		this.emailHandler = emailHandler
	}
	
	MessageHandler create(MessageContext ctx) {
		return new Handler(ctx)
	}
	
	class Handler implements MessageHandler {
		MessageContext ctx
		Email email = new Email()
		
		Handler(MessageContext ctx) {
			this.ctx = ctx
		}
		
		void from(String from) throws RejectException {
			email.smtpFrom = from
		}
		
		void recipient(String recipient) throws RejectException {
			email.smtpTo = recipient
		}
		
		void data(InputStream data) throws IOException {
			Session s = Session.getInstance(new Properties())
			MimeMessage message = new MimeMessage(s, data)
			for (Enumeration<MHeader> e = message.getAllHeaders(); e.hasMoreElements();) {
				MHeader h = e.nextElement()
				email.addToHeaders(name: h.name, value: h.value)
				if (h.name == "To") {
					email.to = h.value
				} else if (h.name == "From") {
					email.from = h.value
				} else if (h.name == "Subject") {
					email.subject = h.value
				}
			}
			
			email.body = message.getContent()
		}
		
		void done() {
			emailHandler.accept(email)
		}
	}
}