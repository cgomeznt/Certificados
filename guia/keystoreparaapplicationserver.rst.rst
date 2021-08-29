Crear un archivo Java KeyStore valido para uso en los navegadores
==============================================================


Los pasos son los siguientes:

* Crear la Private Key y generar el Request.

* Crear el archivo de configuración.

* Firmar el Request para generar el certificado del servidor o servicio.

* Preparar el directorio de trabajo.

* Crear el archivo de formato .p12 con la Private Key.

* Crear el almacén de claves Keystore.

* Importar el certificado dentro del keystore.

* Importar la CA dentro del keystore.



Se debe tener una Entidad Certificadora para crear la llave y el request, para luego firmarlo con el certificado publico de la CA. Ver el siguiente link para crear una Entidad Certificadora (SI aun no la tiene) https://github.com/cgomeznt/Certificados/blob/master/guia/cacentos7.rst
 Esto seria como lo siguiente. 

Crear la Private Key y generar el Request
+++++++++++++++++++++++++++++++++++++++++++++++

Se debe generar un Private Key con el Request::

	# openssl req -newkey rsa:2048 -nodes -keyout keyservice/srvutils.key -out request/srvutils.csr -subj "/C=VE/ST=DC/L=Caracas/O=PERSONAL/OU=TI/CN=srvutils"
	Generating a 2048 bit RSA private key
	.............................................................................................+++
	.................................................................+++
	writing new private key to 'srvutils.key' 
	-----


* La opción "**nodes**" es para que la clave privada no sea protegida con una passphrase. Si el certificado no se utilizara para la autenticación de servidores, no se debería incluir en la opción anterior.
* El "Common Name" (CN) es la información que identifica de forma única al servicio, por lo que debemos asegurarnos de escribirlo correctamente.

Al finalizar se crean dos archivos:

* srvutils.csr: El pedido de certificado.
* private/srvutils.key: La clave privada, que no ha sido protegida con una passphrase.

Se deben crear permisos restrictivos sobre la clave privada::
	
	# chown root.root keyservice/srvutils.key
	# chmod 0400 keyservice/srvutils.key


Crear el archivo de configuración
++++++++++++++++++++++++++++++++++++++++++

Creamos este archivo para tener una administración mas amplia::

	vi conf/srvutils.conf
	[req]
	distinguished_name = req_distinguished_name
	req_extensions = v3_req


	[req_distinguished_name]
	countryName = VE
	countryName_default = VE
	stateOrProvinceName = Distrito Capital
	stateOrProvinceName_default = Distrito Capital
	localityName = Caracas
	localityName_default = Caracas
	organizationName = EMPRESA International CA
	organizationName_default = EMPRESA International CA
	organizationalUnitName	= Criptografia
	organizationalUnitName_default	= Criptografia
	commonName = srvutils.local
	commonName_default = monitoreo.local
	commonName_max	= 64


	[ v3_req ]
	# Extensions to add to a certificate request
	basicConstraints = CA:FALSE
	keyUsage = nonRepudiation, digitalSignature, keyEncipherment
	subjectAltName = @alt_names


	[alt_names]
	DNS.1 = srvutils.local
	DNS.2 = monitoreo.local
	IP.1 = 192.168.0.20



Firmar el Request para generar el certificado del servidor o servicio.
+++++++++++++++++++++++++++++++++++++++++++++++++++

A continuación firmamos el pedido de certificado para generar el certificado para el servidor o servicio::

	# openssl x509 -req -days 185 -extfile conf/srvutils.conf -extensions v3_req -CA certs/CA_empresa.crt -CAkey private/CA_empresa.key -CAserial ca.srl -CAcreateserial -in request/srvutils.csr -out newcerts/srvutils.crt
	Signature ok
	subject=/C=VE/ST=DC/L=Caracas/O=PERSONAL/OU=TI/CN=srvutils
	Getting CA Private Key
	Enter pass phrase for private/CA_empresa.key:



Si se coloca la opción "-policy policy_anything" indica que no se requiere que los campos "Country", "State" o "City", es para que coincidan con los de la CA.

Al finalizar se crean dos nuevos archivos:

* certs/srvutils.crt: Certificado del servidor, que puede hacerse públicamente disponible.

En este momento podemos eliminar el Request del certificado, el cual no necesitaremos más (srvutils.csr)::

	# rm –f request/srvutils.csr 


Preparar el directorio de trabajo
++++++++++++++++++++++++++++

Copiamos la llave y el certificado, también el certificado publico de la CA al directorio de trabajo.::

	# mkdir certificados
	# cd certificados
	# cp /opt/CA/newcerts/srvutils.crt /opt/CA/keyservice/srvutils.key /opt/CA/certs/CA_cursoinfraestructura.crt .

Consultamos los certificados::

	# openssl x509 -in srvutils.crt -noout -text
	# openssl x509 -in CA_empresa.crt -noout -text

Verificamos e certificado::

	# openssl verify -CAfile CA_empresa.crt srvutils.crt


Crear el archivo de formato .p12 con la Private Key
+++++++++++++++++++++++++++

No es posible cargar directamente en un keystore una Private-Key, por eso debemos integrar primero en un formato p12 la private-key y el certificado.::

	# openssl pkcs12 -export -name srvutils-key -in srvutils.crt -inkey srvutils.key -out srvutils.p12

	Enter Export Password: changeit	
	Verifying - Enter Export Password: changeit
	# 

**IMPORTANTE**
**NOTA** En el paso anterior puede colocar cualquier clave y se va crear el .p12 con éxito, pero esta clave debe ser igual a la que se le colocara al Java KeyStore. Si no se homologan las claves el **application server** (Tomcat, JBoss, Glasfish, Weblogi, WebSphere, etc) podrá abrir el Java KeyStore, pero no el Private-Key por tener otra clave.
**IMPORTANTE**

Consultamos el certificado .p12::

	# openssl pkcs12 -info -in srvutils.p12

	Enter Import Password: changeit
	MAC Iteration 2048
	MAC verified OK
	PKCS7 Encrypted data: pbeWithSHA1And40BitRC2-CBC, Iteration 2048
	Certificate bag
	Bag Attributes
	    localKeyID: 4D F3 3E E0 EF 2C 56 F2 39 84 EB 02 E8 CC B3 68 BD 43 AF 77 
	    friendlyName: srvutils-key
	subject=/C=VE/ST=DC/L=Caracas/O=PERSONAL/OU=TI/CN=srvutils
	issuer=/C=VE/ST=DC/L=CCS/O=Default Company Ltd/OU=Sop App/CN=PERSONAL/emailAddress=root@personal.local
	-----BEGIN CERTIFICATE-----
	MIIDvjCCAqagAwIBAgIJALU559uWUDLgMA0GCSqGSIb3DQEBCwUAMIGPMQswCQYD
	VQQGEwJWRTELMAkGA1UECAwCREMxDDAKBgNVBAcMA0NDUzEcMBoGA1UECgwTRGVm
	YXVsdCBDb21wYW55IEx0ZDEQMA4GA1UECwwHU29wIEFwcDERMA8GA1UEAwwIUEVS
	U09OQUwxIjAgBgkqhkiG9w0BCQEWE3Jvb3RAcGVyc29uYWwubG9jYWwwHhcNMjEw
	ODI4MjIyNjE2WhcNMjIwMzAxMjIyNjE2WjBfMQswCQYDVQQGEwJWRTELMAkGA1UE
	CAwCREMxEDAOBgNVBAcMB0NhcmFjYXMxETAPBgNVBAoMCFBFUlNPTkFMMQswCQYD
	VQQLDAJUSTERMA8GA1UEAwwIc3J2dXRpbHMwggEiMA0GCSqGSIb3DQEBAQUAA4IB
	DwAwggEKAoIBAQCwTQXhYX2YueVLZTh2hFq8uB9UwuFhVMZ17si9BiYxD7paPQqg
	RL8zXR7B4w3HgPuMmXaHPHpi5bj6/0MqlEVuCpKHdwSn4j4mu04EBbYTFPCgMl0O
	4T4lYzBRmEGq2gTY377emMiVLy/lBZAqKLy//7VVA+7/oAkUelbpE7qYTGRawWt3
	zTTHEtpRGbAQnseAb7RClP4leOSaWnQ40nH2nSGI0f8FVt+EI6UHaL4Gqe9zEjK1
	ebJzLtMtIEb8OqS/t2b8FAY5tEVaWBqVG1YA29T08wItuxaGcTX/hOKFCVw8wxr6
	VyWFC8v3n/4qtg4u9wS/WoxjSGKY6x5/KQdDAgMBAAGjTDBKMAkGA1UdEwQCMAAw
	CwYDVR0PBAQDAgXgMDAGA1UdEQQpMCeCDnNydnV0aWxzLmxvY2Fsgg9tb25pdG9y
	ZW8ubG9jYWyHBMCoABQwDQYJKoZIhvcNAQELBQADggEBAL+k3lYI4PKmVg/tgL0T
	lxWVRsLm07Re9Q4S0Xn4y1c7o7Xb6oIk9eyarIDOiX/64RvGOs9sjijzRYlm28us
	hzj9iltFZ28TxU9tC5eGS7af/JzeZ6d5guQhZUscoRzCyEyieKXIkJ2TC40EJSrB
	xnrR4JXgw26nEK3e5u8nScyhNgKP8I1AB/mQYKw6RjrFkS+2gMzqV4eijIwkiHjn
	FjB33QO+68DVncDI6cWVXxl9WmhBSbBDEVKWgpDSm3LTtnIAfw9nnaZv/CEulcD9
	+PRHN4CHB5L1DHcHhny4IEJacEQtGo6jOKgb4enx6czvq4kPnHEsh00wqiR9EahN
	7p8=
	-----END CERTIFICATE-----
	PKCS7 Data
	Shrouded Keybag: pbeWithSHA1And3-KeyTripleDES-CBC, Iteration 2048
	Bag Attributes
	    localKeyID: 4D F3 3E E0 EF 2C 56 F2 39 84 EB 02 E8 CC B3 68 BD 43 AF 77 
	    friendlyName: srvutils-key
	Key Attributes: <No Attributes>
	Enter PEM pass phrase: changeit
	Verifying - Enter PEM pass phrase: changeit
	-----BEGIN ENCRYPTED PRIVATE KEY-----
	MIIFDjBABgkqhkiG9w0BBQ0wMzAbBgkqhkiG9w0BBQwwDgQI7OQM2FFT5MYCAggA
	MBQGCCqGSIb3DQMHBAihc4FTSqzSYgSCBMhquF91omAS8wTWIOlPIPOeJ1YW1bdu
	RdzPCoaNUR5ZYyLoF6LEzyZ8U97AewaB/Egg1Vt8++9TgN6oG2HsRSklz2IN3pMq
	v8btyqQNSw6/Lb4LzdL8QdtXvZwknxhR8rATmOU3wAM1pDuUFV/ejGjxoY7BJh/K
	tiHmutQkdI1dZXUIHnvR9bC8VeRe70JCGpUqgNX5GPJjbFbtoWsc52+WlgG7WBit
	PlSD6RZVtMvCJ6FBnMjc2Ny+co8TFAAnTxPstjFpl0uC+4nPk78tiBqGliS5qxpL
	AFwoqGkkZW1XSx93torvhH7JvWOKnE7DT0MxQEDhwISd4h3Zu8O20H0E2u/p8IfW
	btDNGHiEHJ+8M3LaHDboZAsun7nXtRUwCUU7vrBIfMEmSRBuFsXbhAO3ZIR9idUL
	Pc8wBdNaZRmldKz5xPcMxZJy9HeBiKJGB2pWlMS1TdRdXVex4gSWVAoroH7BXYAk
	cC1i9r240DdcWMJB4dc/3mFcaLnDyh1iRhNmYHT69kk7b2bH7BfGBu/fYKjaWqAz
	4a+Jc53uXFlK7QHaDUSO5Hld/rxVpxG8dAOIaXAoOuZD2PG1kuwmbtzFhEU0z35h
	ZmnegPLZJxzZfUsnwPKp8B7e/q715B1pnxSMzS3YBN8BFrdEHEA07bd05utM/LL5
	xQL2EDskACvKU3MGJ2VjTahcHRmEnUx4aX6MeYd2nJNu4jQtMw7bYxbSOx+0Y7+X
	YibCbIVU6wFMTDm8mNGwY8OQS+1yxXBvMFI3M8gOTBMTqZhBXNLg6FfzmSholQuA
	4CCkjGWYUTBarRwFr7LWYpy2J6+2HfB/LFMZV33RynjFPO/TJevlHiuYTJ3FnAes
	UveE/abEUO/oj9kvHVEMhwrmfyH2AbSJ3n7bNucUPNk5sqGu0Bgs0WSsdy7GZQfC
	dj641tU+POr829xVyWhRP+sNg7VDEom7h4jg/7CV4kVrj6An5Yyxgj+ORXYPpK+y
	jb+EWh808+eebC2FMNbqOLZkL8QE3Ls4a3qABQHnpSQeDjh1WwEWJDeFWdOiE4cX
	HndTJX7Nc3fzgw8CsT5DSgoCpOcaWhya5vuYVVgCIpM87BQV0w65H8C6RTMpbIX/
	6AUr1HXjUC0yW7n9UzvMaPgoulVA6OuvoayeSNgtCLZzHwI4at+kJgwLWkJENObs
	C9MnT9Q0TcrFCnLHSIzggonZtfzjAuGdpzp1LP7XhPY+k/Cp0KsPBqu41Eujt2aQ
	/YGPreg54NzbV18ofDOLOmkX291CKcb5B/sh0ycRE7Z1KIyO1QjEeWXV8/GFvE3+
	5WGjl1JXRybvPW83h90ReCrY+0R9iLuLBFLUcN7FVWb0m+xU0K2e7OT5bNUxiKqx
	x9qXAT2e0vNYO6aRxDdCDhQA77iuySXZvQKO4DdEyiYME3b0UXd4EQHyuRfaq7rT
	I/uzK8ZJ7JCLd63TLigIH9m/gdVLAXHZHiANyhju6Q/KOjsoqovzGDKK7Z8M2pSd
	7WCuNXkNNk2ytOLy8htKx+pBE+CIB16NPztWgm7D50nwJ/YJjTt26/Mx2zkYNU60
	nWzGAJ3e1+d228HSk8MpCWaV0n5pEacguIhqt18ehArJAuF4YVx6yspg1hdzJR6l
	Snk=
	-----END ENCRYPTED PRIVATE KEY-----
	# 


Crear el almacén de claves Keystore 
++++++++++++++++++++++++++

Creamos el almacén de claves Keystore agregando el p12 anteriormente::

	# keytool -importkeystore -destkeystore Mykeystore.jks -srckeystore srvutils.p12 -srcstoretype pkcs12 -alias srvutils-key

	Introduzca la contraseña de almacén de claves de destino:  changeit
	Volver a escribir la contraseña nueva: changeit
	Introduzca la contraseña de almacén de claves de origen:  changeit
	# 


Consultamos el keystore y debemos ver las entrada de la private key::

	# keytool -list -v -keystore Mykeystore.jks

	Introduzca la contraseña del almacén de claves:  changeit

	Tipo de Almacén de Claves: JKS
	Proveedor de Almacén de Claves: SUN

	Su almacén de claves contiene 1 entrada

	Nombre de Alias: srvutils-key
	Fecha de Creación: 28/08/2021
	Tipo de Entrada: PrivateKeyEntry
	Longitud de la Cadena de Certificado: 1
	Certificado[1]:
	Propietario: CN=srvutils, OU=TI, O=PERSONAL, L=Caracas, ST=DC, C=VE
	Emisor: EMAILADDRESS=root@personal.local, CN=PERSONAL, OU=Sop App, O=Default Company Ltd, L=CCS, ST=DC, C=VE
	Número de serie: b539e7db965032e0
	Válido desde: Sat Aug 28 18:26:16 EDT 2021 hasta: Tue Mar 01 17:26:16 EST 2022
	Huellas digitales del Certificado:
		 MD5: 03:28:4A:77:CA:9A:D1:4C:AB:44:F7:7D:75:29:88:5C
		 SHA1: 4D:F3:3E:E0:EF:2C:56:F2:39:84:EB:02:E8:CC:B3:68:BD:43:AF:77
		 SHA256: 1E:AD:68:0D:0A:1A:6B:6A:31:1C:DD:56:B3:06:12:B4:F8:60:18:A4:E2:E2:1D:F8:4D:5F:F7:76:14:67:15:05
		 Nombre del Algoritmo de Firma: SHA256withRSA
		 Versión: 3

	Extensiones: 

	#1: ObjectId: 2.5.29.19 Criticality=false
	BasicConstraints:[
	  CA:false
	  PathLen: undefined
	]

	#2: ObjectId: 2.5.29.15 Criticality=false
	KeyUsage [
	  DigitalSignature
	  Non_repudiation
	  Key_Encipherment
	]

	#3: ObjectId: 2.5.29.17 Criticality=false
	SubjectAlternativeName [
	  DNSName: srvutils.local
	  DNSName: monitoreo.local
	  IPAddress: 192.168.0.20
	]



	*******************************************
	*******************************************

Agregar el certificado dentro del keystore
+++++++++++++++++++++++++++++++++++++++

Agregamos la certificado dentro del keystore::

	# keytool -import -alias srvutils-certificate -file CA_empresa.crt -keystore Mykeystore.jks

	Introduzca la contraseña del almacén de claves:  changeit
	El certificado ya existe en el almacén de claves con el alias <srvutils-key>
	¿Aún desea agregarlo? [no]:  s
	Se ha agregado el certificado al almacén de cAgregar la CA dentro del keystorelaves
	# 

Consultamos el keystore y debemos ver las entrada de la private key y el certificado::

	# keytool -list -v -keystore Mykeystore.jks

	Introduzca la contraseña del almacén de claves:  

	Tipo de Almacén de Claves: JKS
	Proveedor de Almacén de Claves: SUN

	Su almacén de claves contiene 2 entradas

	Nombre de Alias: srvutils-certificate
	Fecha de Creación: 28/08/2021
	Tipo de Entrada: trustedCertEntry

	Propietario: CN=srvutils, OU=TI, O=PERSONAL, L=Caracas, ST=DC, C=VE
	Emisor: EMAILADDRESS=root@personal.local, CN=PERSONAL, OU=Sop App, O=Default Company Ltd, L=CCS, ST=DC, C=VE
	Número de serie: b539e7db965032e0
	Válido desde: Sat Aug 28 18:26:16 EDT 2021 hasta: Tue Mar 01 17:26:16 EST 2022
	Huellas digitales del Certificado:
		 MD5: 03:28:4A:77:CA:9A:D1:4C:AB:44:F7:7D:75:29:88:5C
		 SHA1: 4D:F3:3E:E0:EF:2C:56:F2:39:84:EB:02:E8:CC:B3:68:BD:43:AF:77
		 SHA256: 1E:AD:68:0D:0A:1A:6B:6A:31:1C:DD:56:B3:06:12:B4:F8:60:18:A4:E2:E2:1D:F8:4D:5F:F7:76:14:67:15:05
		 Nombre del Algoritmo de Firma: SHA256withRSA
		 Versión: 3

	Extensiones: 

	#1: ObjectId: 2.5.29.19 Criticality=false
	BasicConstraints:[
	  CA:false
	  PathLen: undefined
	]

	#2: ObjectId: 2.5.29.15 Criticality=false
	KeyUsage [
	  DigitalSignature
	  Non_repudiation
	  Key_Encipherment
	]

	#3: ObjectId: 2.5.29.17 Criticality=false
	SubjectAlternativeName [
	  DNSName: srvutils.local
	  DNSName: monitoreo.local
	  IPAddress: 192.168.0.20
	]



	*******************************************
	*******************************************


	Nombre de Alias: srvutils-key
	Fecha de Creación: 28/08/2021
	Tipo de Entrada: PrivateKeyEntry
	Longitud de la Cadena de Certificado: 1
	Certificado[1]:
	Propietario: CN=srvutils, OU=TI, O=PERSONAL, L=Caracas, ST=DC, C=VE
	Emisor: EMAILADDRESS=root@personal.local, CN=PERSONAL, OU=Sop App, O=Default Company Ltd, L=CCS, ST=DC, C=VE
	Número de serie: b539e7db965032e0
	Válido desde: Sat Aug 28 18:26:16 EDT 2021 hasta: Tue Mar 01 17:26:16 EST 2022
	Huellas digitales del Certificado:
		 MD5: 03:28:4A:77:CA:9A:D1:4C:AB:44:F7:7D:75:29:88:5C
		 SHA1: 4D:F3:3E:E0:EF:2C:56:F2:39:84:EB:02:E8:CC:B3:68:BD:43:AF:77
		 SHA256: 1E:AD:68:0D:0A:1A:6B:6A:31:1C:DD:56:B3:06:12:B4:F8:60:18:A4:E2:E2:1D:F8:4D:5F:F7:76:14:67:15:05
		 Nombre del Algoritmo de Firma: SHA256withRSA
		 Versión: 3

	Extensiones: 

	#1: ObjectId: 2.5.29.19 Criticality=false
	BasicConstraints:[
	  CA:false
	  PathLen: undefined
	]

	#2: ObjectId: 2.5.29.15 Criticality=false
	KeyUsage [
	  DigitalSignature
	  Non_repudiation
	  Key_Encipherment
	]

	#3: ObjectId: 2.5.29.17 Criticality=false
	SubjectAlternativeName [
	  DNSName: srvutils.local
	  DNSName: monitoreo.local
	  IPAddress: 192.168.0.20
	]



	*******************************************
	*******************************************


Agregar la CA dentro del keystore
++++++++++++++++++++++++++++++++++++

Agregar el certificado publico de la CA dentro del keystore::

	# keytool -import -alias CA-root -file CA_empresa.crt -keystore Mykeystore.jks

	Introduzca la contraseña del almacén de claves:  changeit
	Propietario: EMAILADDRESS=root@personal.local, CN=PERSONAL, OU=Sop App, O=Default Company Ltd, L=CCS, ST=DC, C=VE
	Emisor: EMAILADDRESS=root@personal.local, CN=PERSONAL, OU=Sop App, O=Default Company Ltd, L=CCS, ST=DC, C=VE
	Número de serie: ddff243bcbceacc1
	Válido desde: Mon Aug 23 15:06:20 EDT 2021 hasta: Thu Aug 21 15:06:20 EDT 2031
	Huellas digitales del Certificado:
		 MD5: 04:97:A4:4A:90:BB:F1:14:DE:FD:BE:36:15:59:4B:12
		 SHA1: 79:99:36:30:82:93:04:A0:DA:C4:E6:C3:F3:A5:63:84:57:A4:AF:CF
		 SHA256: 3E:D6:5B:0A:8D:FA:F5:70:CB:D1:DB:65:24:1D:E5:4A:A1:E1:F4:71:C8:18:BA:22:2C:CF:C7:AA:64:ED:50:67
		 Nombre del Algoritmo de Firma: SHA256withRSA
		 Versión: 3

	Extensiones: 

	#1: ObjectId: 2.5.29.35 Criticality=false
	AuthorityKeyIdentifier [
	KeyIdentifier [
	0000: 8D 43 A0 20 E3 1C EB F6   C5 F7 E6 1D DB D2 8E 61  .C. ...........a
	0010: F7 B6 AA 84                                        ....
	]
	]

	#2: ObjectId: 2.5.29.19 Criticality=false
	BasicConstraints:[
	  CA:true
	  PathLen:2147483647
	]

	#3: ObjectId: 2.5.29.14 Criticality=false
	SubjectKeyIdentifier [
	KeyIdentifier [
	0000: 8D 43 A0 20 E3 1C EB F6   C5 F7 E6 1D DB D2 8E 61  .C. ...........a
	0010: F7 B6 AA 84                                        ....
	]
	]

	¿Confiar en este certificado? [no]:  s
	Se ha agregado el certificado al almacén de claves


Consultamos el keystore y debemos ver las entrada de la private key,l certificado y el certificado publico de la CA::

	# keytool -list -v -keystore Mykeystore.jks

	Introduzca la contraseña del almacén de claves:  

	Tipo de Almacén de Claves: JKS
	Proveedor de Almacén de Claves: SUN

	Su almacén de claves contiene 3 entradas

	Nombre de Alias: srvutils-certificate
	Fecha de Creación: 28/08/2021
	Tipo de Entrada: trustedCertEntry

	Propietario: CN=srvutils, OU=TI, O=PERSONAL, L=Caracas, ST=DC, C=VE
	Emisor: EMAILADDRESS=root@personal.local, CN=PERSONAL, OU=Sop App, O=Default Company Ltd, L=CCS, ST=DC, C=VE
	Número de serie: b539e7db965032e0
	Válido desde: Sat Aug 28 18:26:16 EDT 2021 hasta: Tue Mar 01 17:26:16 EST 2022
	Huellas digitales del Certificado:
		 MD5: 03:28:4A:77:CA:9A:D1:4C:AB:44:F7:7D:75:29:88:5C
		 SHA1: 4D:F3:3E:E0:EF:2C:56:F2:39:84:EB:02:E8:CC:B3:68:BD:43:AF:77
		 SHA256: 1E:AD:68:0D:0A:1A:6B:6A:31:1C:DD:56:B3:06:12:B4:F8:60:18:A4:E2:E2:1D:F8:4D:5F:F7:76:14:67:15:05
		 Nombre del Algoritmo de Firma: SHA256withRSA
		 Versión: 3

	Extensiones: 

	#1: ObjectId: 2.5.29.19 Criticality=false
	BasicConstraints:[
	  CA:false
	  PathLen: undefined
	]

	#2: ObjectId: 2.5.29.15 Criticality=false
	KeyUsage [
	  DigitalSignature
	  Non_repudiation
	  Key_Encipherment
	]

	#3: ObjectId: 2.5.29.17 Criticality=false
	SubjectAlternativeName [
	  DNSName: srvutils.local
	  DNSName: monitoreo.local
	  IPAddress: 192.168.0.20
	]



	*******************************************
	*******************************************


	Nombre de Alias: srvutils-key
	Fecha de Creación: 28/08/2021
	Tipo de Entrada: PrivateKeyEntry
	Longitud de la Cadena de Certificado: 1
	Certificado[1]:
	Propietario: CN=srvutils, OU=TI, O=PERSONAL, L=Caracas, ST=DC, C=VE
	Emisor: EMAILADDRESS=root@personal.local, CN=PERSONAL, OU=Sop App, O=Default Company Ltd, L=CCS, ST=DC, C=VE
	Número de serie: b539e7db965032e0
	Válido desde: Sat Aug 28 18:26:16 EDT 2021 hasta: Tue Mar 01 17:26:16 EST 2022
	Huellas digitales del Certificado:
		 MD5: 03:28:4A:77:CA:9A:D1:4C:AB:44:F7:7D:75:29:88:5C
		 SHA1: 4D:F3:3E:E0:EF:2C:56:F2:39:84:EB:02:E8:CC:B3:68:BD:43:AF:77
		 SHA256: 1E:AD:68:0D:0A:1A:6B:6A:31:1C:DD:56:B3:06:12:B4:F8:60:18:A4:E2:E2:1D:F8:4D:5F:F7:76:14:67:15:05
		 Nombre del Algoritmo de Firma: SHA256withRSA
		 Versión: 3

	Extensiones: 

	#1: ObjectId: 2.5.29.19 Criticality=false
	BasicConstraints:[
	  CA:false
	  PathLen: undefined
	]

	#2: ObjectId: 2.5.29.15 Criticality=false
	KeyUsage [
	  DigitalSignature
	  Non_repudiation
	  Key_Encipherment
	]

	#3: ObjectId: 2.5.29.17 Criticality=false
	SubjectAlternativeName [
	  DNSName: srvutils.local
	  DNSName: monitoreo.local
	  IPAddress: 192.168.0.20
	]



	*******************************************
	*******************************************


	Nombre de Alias: ca-root
	Fecha de Creación: 28/08/2021
	Tipo de Entrada: trustedCertEntry

	Propietario: EMAILADDRESS=root@personal.local, CN=PERSONAL, OU=Sop App, O=Default Company Ltd, L=CCS, ST=DC, C=VE
	Emisor: EMAILADDRESS=root@personal.local, CN=PERSONAL, OU=Sop App, O=Default Company Ltd, L=CCS, ST=DC, C=VE
	Número de serie: ddff243bcbceacc1
	Válido desde: Mon Aug 23 15:06:20 EDT 2021 hasta: Thu Aug 21 15:06:20 EDT 2031
	Huellas digitales del Certificado:
		 MD5: 04:97:A4:4A:90:BB:F1:14:DE:FD:BE:36:15:59:4B:12
		 SHA1: 79:99:36:30:82:93:04:A0:DA:C4:E6:C3:F3:A5:63:84:57:A4:AF:CF
		 SHA256: 3E:D6:5B:0A:8D:FA:F5:70:CB:D1:DB:65:24:1D:E5:4A:A1:E1:F4:71:C8:18:BA:22:2C:CF:C7:AA:64:ED:50:67
		 Nombre del Algoritmo de Firma: SHA256withRSA
		 Versión: 3

	Extensiones: 

	#1: ObjectId: 2.5.29.35 Criticality=false
	AuthorityKeyIdentifier [
	KeyIdentifier [
	0000: 8D 43 A0 20 E3 1C EB F6   C5 F7 E6 1D DB D2 8E 61  .C. ...........a
	0010: F7 B6 AA 84                                        ....
	]
	]

	#2: ObjectId: 2.5.29.19 Criticality=false
	BasicConstraints:[
	  CA:true
	  PathLen:2147483647
	]

	#3: ObjectId: 2.5.29.14 Criticality=false
	SubjectKeyIdentifier [
	KeyIdentifier [
	0000: 8D 43 A0 20 E3 1C EB F6   C5 F7 E6 1D DB D2 8E 61  .C. ...........a
	0010: F7 B6 AA 84                                        ....
	]
	]



	*******************************************
	*******************************************





Publicar el Java KeyStore
++++++++++++++++++++++++

El Java KeyStore creado ahora debe ser entregado y publicado dentro del **Application Server** (Tomcat, JBoss, Glasfish, Weblogic, WebSphere, etc.)




openssl pkcs12 -export -name srvutils.local -in srvutils.crt -inkey srvutils.key -chain -CAfile CA_empresa.crt -out srvutils.p12

keytool -importkeystore -deststorepass changeit -destkeystore keystore.jks -srckeystore srvutils.p12 -srcstoretype PKCS12

openssl pkcs12 -export -name srvutils-private-key -in srvutils.crt -inkey srvutils.key -out srvutils.p12

openssl pkcs12 -info -in srvutils.p12

keytool -delete -noprompt -alias srvutils-key  -keystore keystore.jks -storepass changeit

Use Java keytool to convert from JKS to P12...
==================

Export from keytool's proprietary format (called "JKS") to standardized format PKCS #12:

	keytool -importkeystore \
	    -srckeystore keystore.jks \
	    -destkeystore keystore.p12 \
	    -deststoretype PKCS12 \
	    -srcalias <jkskeyalias> \
	    -deststorepass <password> \
	    -destkeypass <password>

keytool -importkeystore \
	    -srckeystore keystore.jks \
	    -destkeystore srvutils.p12 \
	    -deststoretype PKCS12 \
	    -srcalias srvutils-key \
	    -deststorepass venezuela21 \
	    -destkeypass venezuela21

...then use openssl to export from P12 to PEM
Export certificate using openssl::

	openssl pkcs12 -in keystore.p12  -nokeys -out srvutils.key

Export unencrypted private key::

	openssl pkcs12 -in keystore.p12  -nodes -nocerts -out key.pem

Export the certificate.

	keytool -export -alias teiid -keystore server.keystore -rfc -file public.cert




keytool -importkeystore -destkeystore keystore.jks -srckeystore srvutils.p12 -srcstoretype pkcs12 -alias srvutils-private-key

keytool -import -alias srvutils-certificate -file srvutils.crt  -keystore keystore.jks -storepass changeit 

keytool -import -trustcacerts -alias ca-certificate -file CA_empresa.crt -keystore keystore.jks -storepass changeit
