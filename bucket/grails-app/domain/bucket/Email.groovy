package bucket

import grails.compiler.GrailsCompileStatic

@GrailsCompileStatic
class Email {
	Long received = System.currentTimeSeconds()
	String smtpTo
	String smtpFrom
	String payload
	
	static constraints = {
		received nullable: false
		smtpTo nullable: false
		smtpFrom nullable: false
		payload nullable: false
	}
	
	static mapping = {
		smtpTo type: "text"
		smtpFrom type: "text"
		payload type: "text"
	}
}