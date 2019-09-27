package bucket

import grails.compiler.GrailsCompileStatic
import groovy.util.logging.Slf4j
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
	
	@Slf4j
	class Handler implements MessageHandler {
		MessageContext ctx
		Email email = new Email()
		
		Handler(MessageContext ctx) {
			log.trace("Handler(...)")
			this.ctx = ctx
		}
		
		void from(String from) throws RejectException {
			log.trace("from(from:$from)")
			email.smtpFrom = from
		}
		
		void recipient(String recipient) throws RejectException {
			log.trace("recipient(recipient:$recipient)")
			email.smtpTo = recipient
		}
		
		void data(InputStream dataStream) throws IOException {
			email.payload = Utils.convertStreamToString(dataStream)
			log.trace("email.payload: $email.payload")
		}
		
		void done() {
			log.trace("done()")
			emailHandler.accept(email)
		}
	}
}
