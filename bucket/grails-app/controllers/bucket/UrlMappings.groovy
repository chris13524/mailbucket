package bucket

class UrlMappings {
	static mappings = {
		"/"(view: "/home")
		get "/$emailAddress"(controller: "email", action: "retrieve")
		
		"404"(view: "/clientError/notFound")
		"500"(view: "/serverError/internalSeverError")
	}
}
