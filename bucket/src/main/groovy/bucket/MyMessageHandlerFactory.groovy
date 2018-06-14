package bucket

import grails.compiler.GrailsCompileStatic
import org.subethamail.smtp.MessageContext
import org.subethamail.smtp.MessageHandler
import org.subethamail.smtp.MessageHandlerFactory
import org.subethamail.smtp.RejectException

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
		
		void data(InputStream dataStream) throws IOException {
			email.payload = Utils.convertStreamToString(dataStream)
		}
		
		void done() {
			emailHandler.accept(email)
		}
	}
}