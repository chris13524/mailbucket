package bucket

import grails.compiler.GrailsCompileStatic

@GrailsCompileStatic
class EmailController {
	EmailService emailService
	
	def retrieve() {
		String emailAddress = params.emailAddress
		if (emailAddress == null) {
			response.sendError(400)
			return
		}
		
		Email email = emailService.popLatestEmail(emailAddress)
		
		if (email == null) {
			response.sendError(404)
			return []
		}
		
		return [email: email]
	}
}