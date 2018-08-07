# Mail Bucket

For more information, see [mailbucket.io](https://mailbucket.io).

## API

My goal designing this API was to make it as simple as possible to retrieve an email and check its contents. This results in violations of principles such as "GET shouldn't modify", but I don't care.

First, make up an @mailbucket.io address (e.g. xyz@mailbucket.io) and send an email to it.

Next, perform a GET request to: https://mailbucket.io/xyz@mailbucket.io

You should get back a JSON response containing these fields:

 - smtpTo - the address that SMTP deems the recipient
 - smtpFrom - the address that SMTP deems the sender
 - raw - the raw email (parse it yourself?)
 - to - an alias for the 'To' header
 - from - an alias for the 'From' header
 - subject - an alias for the 'Subject' header
 - headers - an map of headers and their values (does not support duplicates)
 - rawHeaders - a list of name/value pairs (supports duplicates)
 - body - for plain text emails the text itself, for multipart this will be a map of content types and the corresponding body
 - rawBody - for plain text emails the text itself, for multipart this will be a list of the content type, headers, and body
 - text - if multipart, the text/plain part
 - html - if multipart, the text/html part

## Host myself?
Sure! Just install Docker and Docker Compose, adjust the hostname in the `Caddyfile`, and run `docker-compose up -d`.

## TODO
 - easier API to retrieve attachements
 - filtering API response
 - TypeScript class (to document the API better)