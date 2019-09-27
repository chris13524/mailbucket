<!DOCTYPE html>
<html>
<head>
	<meta charset="utf-8">
	<meta name="viewport" content="width=device-width, initial-scale=1, shrink-to-fit=no">
	<title>Mail Bucket</title>
	<link rel="stylesheet" href="https://maxcdn.bootstrapcdn.com/bootstrap/4.0.0/css/bootstrap.min.css"
	      integrity="sha384-Gn5384xqQ1aoWXA+058RXPxPg6fy4IWvTNh0E263XmFcJlSAwiGgFAW/dAiS6JXm" crossorigin="anonymous">
</head>

<body>

<div class="container" style="margin-top:1rem;">
	<h1 style="margin-bottom:0;">Mail Bucket</h1>
	
	<p class="text-muted">
		A <a href="https://hichris.com">Chris Smith</a> project.
	</p>
	
	<p>Mail Bucket is a simple tool that makes testing your mail implementation really easy.</p>
	
	<h3>How does it work?</h3>
	
	<p>
		Simply send an email to some @mailbucket.io address. Then perform a GET request to <code>https://mailbucket.io/&lt;address&gt;</code> to retrieve the email. There is no need to configure a mailbox beforehand.
	</p>
	
	<p>
		My goal designing this API was to make it as simple as possible to retrieve an email and check its contents. This results in violations of principles such as "GET shouldn't modify", but I don't care.
	</p>
	
	<h3>Show me an example!</h3>
	
	<ol>
		<li>Send an email to <code><span class="exampleAddress">mytest</span>@mailbucket.io</code></li>
		<li><code>curl https://mailbucket.io/<span class="exampleAddress">mytest</span>@mailbucket.io</code></li>
	</ol>
	
	<h3>Isn't this insecure?</h3>
	
	<p>
		If you use a simple address like <code>mytest@mailbucket.io</code>, then anybody can read your emails and there is always the possibility of two developers picking the same email for their tests. Instead, you should make up a random address such as
		<code><span class="exampleAddress">Y1IhwHzkdnY3z8</span>@mailbucket.io</code>
		and use that. This address should be generated on-the-fly for each test, rather than hard coding it.
	</p>
	
	<p>
		This doesn't currently support SSL delivery (so your emails could be read by men-in-the-middle). But in reality, you shouldn't be sending me security sensitive stuff anyways!
	</p>
	
	<h3>Technicalities</h3>
	
	<ul>
		<li>If the email hasn't been received yet, the request will block for up to 60 seconds until the email is actually delivered. Feel free to make the request again if your mail sending implementation is <i>that</i> slow.
		</li>
		<li>The email can only be retrieved once. Additional requests will block as described above.</li>
		<li>Unretrieved emails are deleted after 1 hour.</li>
		<li>If a second email comes in, it will overwrite the already existing one.</li>
		<li>Retrieval is, by default, case insensitive. If you want to be explicit, provide the <code>?matchCase=true</code> URL parameter.</li>
		<li>Remember to URL encode if your email addresses have any special characters!</li>
		<li>You can use a different domain than mailbucket.io, if you like. Just be sure that somewhere down the line you're sending stuff to this server.</li>
		<li>You can also checkout the source over on <a href="https://github.com/chris13524/mailbucket">GitHub</a>.</li>
	</ul>
</div>

<script>
	function gen() {
		var array = [
			"a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m",
			"n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z",
			"A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M",
			"N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z"];
		var result = "";
		for (var i = 0; i < 8; i++) {
			result += array[Math.floor(Math.random() * array.length)];
		}
		return result;
	}
	
	var address = gen();
	var elements = document.getElementsByClassName("exampleAddress");
	for (var i = 0; i < elements.length; i++) {
		elements[i].innerHTML = address;
	}
</script>

</body>
</html>
