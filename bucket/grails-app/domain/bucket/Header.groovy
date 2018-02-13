package bucket

import grails.compiler.GrailsCompileStatic

@GrailsCompileStatic
class Header {
	String name
	String value
	
	static constraints = {
		name nullable: false
		value nullable: false
	}
	
	static mapping = {
		name type: "text"
		value type: "text"
	}
}