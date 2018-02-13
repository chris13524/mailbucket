package bucket

import com.github.rholder.retry.*
import com.google.common.base.Predicates
import grails.compiler.GrailsCompileStatic
import grails.converters.JSON
import grails.gorm.transactions.Transactional
import org.subethamail.smtp.server.SMTPServer

import java.util.concurrent.Executors
import java.util.concurrent.ScheduledExecutorService
import java.util.concurrent.TimeUnit

@GrailsCompileStatic
@Transactional
class EmailService {
	private static final Integer EXPIRE_AFTER = 60
	private static final Retryer<Email> emailRetryer = RetryerBuilder.<Email> newBuilder()
			.retryIfResult(Predicates.isNull())
			.withWaitStrategy(WaitStrategies.fixedWait(1, TimeUnit.SECONDS))
			.withStopStrategy(StopStrategies.stopAfterDelay(60, TimeUnit.SECONDS))
			.build()
	
	private SMTPServer smtpServer
	private ScheduledExecutorService scheduler
	
	void pushEmail(Email email) {
		log.info("push email: " + (email as JSON))
		Email.findAllBySmtpTo(email.smtpTo, [lock: true]).forEach({ Email e ->
			log.info("overwriting email: " + (email as JSON))
			e.delete(flush: true)
		})
		email.save(flush: true)
	}
	
	void deleteExpiredEmails() {
		Email.findAllByReceivedLessThan(System.currentTimeSeconds() - EXPIRE_AFTER, [lock: true]).forEach({ Email email ->
			log.info("expire email: " + (email as JSON))
			email.delete(flush: true)
		})
	}
	
	Email popLatestEmail(String address) {
		try {
			emailRetryer.call({
				Email email = Email.findBySmtpTo(address, [lock: true])
				
				if (email != null) {
					email.headers.forEach({ String s, String s2 ->
						// this fetches the headers cus I can't get eager fetching to work with H2
					})
					
					log.info("pop email: " + (email as JSON))
					
					email.delete(flush: true)
				}
				
				return email
			})
		} catch (RetryException e) {
			return null
		}
	}
	
	void init() {
		// setup message server
		MyMessageHandlerFactory myFactory = new MyMessageHandlerFactory({ Email email ->
			pushEmail(email)
		})
		smtpServer = new SMTPServer(myFactory)
		smtpServer.setPort(25)
		smtpServer.start()
		
		// setup expiration service
		scheduler = Executors.newScheduledThreadPool(1)
		scheduler.scheduleAtFixedRate({
			deleteExpiredEmails()
		}, 0, 1, TimeUnit.MINUTES)
	}
	
	void destroy() {
		smtpServer.stop()
		scheduler.shutdown()
	}
}