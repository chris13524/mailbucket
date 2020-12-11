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
	private static final Integer EXPIRE_AFTER = 60 * 60
	private static final Retryer<Email> emailRetryer = RetryerBuilder.<Email> newBuilder()
			.retryIfResult(Predicates.isNull())
			.withWaitStrategy(WaitStrategies.fixedWait(1, TimeUnit.SECONDS))
			.withStopStrategy(StopStrategies.stopAfterDelay(60, TimeUnit.SECONDS))
			.build()
	
	private SMTPServer smtpServer
	private ScheduledExecutorService scheduler
	
	ConfigService configService
	
	void pushEmail(Email email) {
		log.debug("pushEmail(email:" + (email as JSON) + ")")
		log.info("push email from: $email.smtpFrom")
		Email.findAllBySmtpTo(email.smtpTo, [lock: true]).forEach({ Email e ->
			log.debug("overwriting email: " + (email as JSON))
			e.delete(flush: true)
		})
		email.save(flush: true)
	}
	
	void deleteExpiredEmails() {
		Email.findAllByReceivedLessThan(System.currentTimeSeconds() - EXPIRE_AFTER, [lock: true]).forEach({ Email email ->
			log.debug("expire email: " + (email as JSON))
			email.delete(flush: true)
		})
	}
	
	Email popLatestEmail(String address, boolean matchCase = false) {
		log.trace("popLatestEmail(address:$address, matchCase:$matchCase)")
		try {
			emailRetryer.call({
				Email email
				if (matchCase) {
					email = Email.findBySmtpTo(address, [lock: true])
				} else {
					String escaped = address
							.replaceAll("%", "\\\\%")
							.replaceAll("_", "\\\\_")
					email = Email.findBySmtpToIlike(escaped, [lock: true])
				}
				
				if (email != null) {
					log.info("pop email from: $email.smtpFrom")
					log.debug("pop email: " + (email as JSON))
					email.delete(flush: true)
				}
				
				return email
			})
		} catch (RetryException ignored) {
			return null
		}
	}
	
	void init() {
		// setup message server
		MyMessageHandlerFactory myFactory = new MyMessageHandlerFactory({ Email email ->
			pushEmail(email)
		})
		smtpServer = new SMTPServer(myFactory)
		smtpServer.setPort(configService.smtpPort as Integer)
		smtpServer.start()
		
		// setup expiration service
		scheduler = Executors.newScheduledThreadPool(1)
		scheduler.scheduleAtFixedRate({
			deleteExpiredEmails()
		}, 0, 5, TimeUnit.MINUTES)
	}
	
	void destroy() {
		smtpServer.stop()
		scheduler.shutdown()
		scheduler.awaitTermination(1, TimeUnit.MINUTES)
	}
}
