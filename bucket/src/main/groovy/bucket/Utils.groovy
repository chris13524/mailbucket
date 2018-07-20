package bucket

final class Utils {
	private Utils() {}
	
	public static String convertStreamToString(InputStream is) {
		BufferedReader reader = new BufferedReader(new InputStreamReader(is))
		StringBuilder sb = new StringBuilder()
		
		String line
		while ((line = reader.readLine()) != null) {
			sb.append(line + "\n")
		}
		return sb.toString()
	}
}