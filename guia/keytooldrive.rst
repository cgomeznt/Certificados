Manejo de certificados con keytool
======================================


Generate the private – public key pair. For demonstration we would use keytool java utility to do so. However we can use other utilities like openssl etc.::

	keytool -genkey -alias DUMMY -keyalg  RSA -keysize 2048 -keystore ./MYKeystore.jks -storetype jks -storepass changeit -keypass changeit

Generate a Certificate Signing Request (CSR) and send it to Certifying Authority.::

	keytool -certreq -keyalg RSA -keysize 2048 -alias DUMMY -file certreq.csr -keystore MYKeystore.jks -storepass changeit

The CA would return with the certificate reply and the RootCA and sometimes an intermediateCA certificate.::

	openssl x509 -req -days 365 -sha1 -extensions v3_req -CA certs/ca.cer -CAkey private/cakey.pem -CAserial ca.srl -CAcreateserial -in certreq.csr -out certs/certreq.cer

Convert the cer to pem, remember if you want imort a key, is in a pem.::

	cat certreq.cer > certreq.pem

Import the certificates into the keystore, this can be done in two ways either by importing the certificates in an order of RootCA, intermediateCA and then Certificate reply. Or we can create a certificate chain clubbing them in an order into a .pem file.::

	keytool -import -file certreq.pem -alias DUMMY -keystore MYKeystore.jks -storepass changeit

To verify the contents of the keystore, you can use the below command,::

	keytool –list –v –keystore MYKeystore.jks -storepass  changeit




