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
	String original
	
	static hasMany = [headers: Header]
	
	static constraints = {
		received nullable: false
		smtpTo nullable: false
		smtpFrom nullable: false
		to nullable: true
		from nullable: true
		subject nullable: true
		body nullable: false
		original nullable: false
	}
	
	static mapping = {
		smtpTo type: "text"
		smtpFrom type: "text"
		to column: "`to`", type: "text"
		from column: "`from`", type: "text"
		subject column: "`subject`", type: "text"
		body type: "text"
		original type: "text"
	}
}