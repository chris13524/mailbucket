# Mail Bucket

For more information, see [mailbucket.io](https://mailbucket.io).

## API response fields

 - smtpTo - the address that SMTP deems the recipient
 - smtpFrom - the address that SMTP deems the sender
 - to - an alias for the 'To' header field
 - from - an alias for the 'From' header field
 - subject - an alias for the 'Subject' header field
 - headers - an object representing the header fields

## TODO
 - support for multipart
 - support for possible duplicate header fields
 - store original email content

## Host myself?
Sure! Just install Docker and Docker Compose and run `docker-compose up -d`.