Newer versions of OpenSSL say BEGIN PRIVATE KEY because they contain the private key + an OID that identifies the key type (this is known as PKCS8 format). To get the old style key (known as either PKCS1 or traditional OpenSSL format) you can do this:

Con esto tomas un key que tenga clave y la clave se la introduce en el nuevo key

	openssl rsa -in server.key -out server_new.key

Alternately, if you have a PKCS1 key and want PKCS8:

	openssl pkcs8 -topk8 -nocrypt -in privkey.pem
