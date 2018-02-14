package bucket

import grails.plugins.mail.MailService
import grails.testing.mixin.integration.Integration
import spock.lang.Specification

@Integration
class EmailSpec extends Specification {
	EmailService emailService
	MailService mailService
	
	void "send and then get"() {
		given:
		mailService.sendMail {
			to "test1@mailbucket.io"
			from "from@example.com"
			subject "test1subject"
			text "test1text"
		}
		
		when:
		Email email = emailService.popLatestEmail("test1@mailbucket.io")
		
		println(email.body)
		
		then:
		email != null
		email.to == "test1@mailbucket.io"
		email.from == "from@example.com"
		email.smtpTo == "test1@mailbucket.io"
		email.smtpFrom == "from@example.com"
		email.subject == "test1subject"
		email.body.trim() == "test1text"
		email.original.contains("To: test1@mailbucket.io")
	}
	
	void "send async and then get"() {
		given:
		mailService.sendMail {
			async true
			to "test1@mailbucket.io"
			from "from@example.com"
			subject "test1subject"
			text "test1text"
		}
		
		when:
		Email email = emailService.popLatestEmail("test1@mailbucket.io")
		
		println(email.body)
		
		then:
		email != null
		email.to == "test1@mailbucket.io"
		email.from == "from@example.com"
		email.smtpTo == "test1@mailbucket.io"
		email.smtpFrom == "from@example.com"
		email.subject == "test1subject"
		email.body.trim() == "test1text"
		email.original.contains("To: test1@mailbucket.io")
	}
	
	void "send async after delay and then get"() {
		given:
		new Thread({
			sleep(5000)
			mailService.sendMail {
				to "test1@mailbucket.io"
				from "from@example.com"
				subject "test1subject"
				text "test1text"
			}
		}).start()
		
		when:
		Email email = emailService.popLatestEmail("test1@mailbucket.io")
		
		println(email.body)
		
		then:
		email != null
		email.to == "test1@mailbucket.io"
		email.from == "from@example.com"
		email.smtpTo == "test1@mailbucket.io"
		email.smtpFrom == "from@example.com"
		email.subject == "test1subject"
		email.body.trim() == "test1text"
		email.original.contains("To: test1@mailbucket.io")
	}
}