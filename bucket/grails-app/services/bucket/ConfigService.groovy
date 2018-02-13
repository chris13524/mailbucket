package bucket

class ConfigService {
	def grailsApplication
	
	String getSmtpPort() {
		return grailsApplication.config.smtpPort
	}
}