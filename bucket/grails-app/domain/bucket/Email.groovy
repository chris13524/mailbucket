package bucket

import grails.compiler.GrailsCompileStatic

@GrailsCompileStatic
class Email {
	Long received = System.currentTimeSeconds()
	String smtpTo
	String smtpFrom
	String to
	String from
	String subject
	String body
	Map<String, String> headers = [:]
	
	static constraints = {
		smtpTo nullable: false
		smtpFrom nullable: false
		to nullable: false
		from nullable: false
		subject nullable: false
		body nullable: false
		received nullable: false
	}
	
	static mapping = {
		smtpTo type: "text"
		smtpFrom type: "text"
		to column: "`to`", type: "text"
		from column: "`from`", type: "text"
		subject column: "`subject`", type: "text"
		body type: "text"
	}
}