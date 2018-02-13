package bucket

class BootStrap {
	EmailService emailService
	
	def init = { servletContext ->
		emailService.init()
	}
	def destroy = {
		emailService.destroy()
	}
}
