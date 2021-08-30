Crear un archivo Java KeyStore valido para uso en los navegadores
==============================================================

Vamos a crear una private key, el request y se va firmar con una CA de la empresa, luego se van a introducir en un archivo Java Keystores la private key, el certificado y el certificado publico de la CA. El Java KeyStore será publicado dentro del **Application Server** (Tomcat, JBoss, Glasfish, Weblogic, WebSphere, etc.), posteriormente vamos a configurar para que desde los navegadores se puedan visualizar el certificado como valido.

Los pasos son los siguientes:

* Crear la Private Key y generar el Request.

* Crear el archivo de configuración.

* Firmar el Request para generar el certificado del servidor o servicio.

* Preparar el directorio de trabajo.

* Crear el archivo de formato .p12 con la Private Key.

* Crear el almacén de claves Keystore.

* Importar el certificado dentro del keystore.

* Importar la CA dentro del keystore.

* Publicar el Java KeyStore en un Application Server.

* Preparar el Navegador para que pueda ver el certificado como valido.



Se debe tener una Entidad Certificadora para crear la llave y el request, para luego firmarlo con el certificado publico de la CA. Ver el siguiente link para crear una Entidad Certificadora (SI aun no la tiene) https://github.com/cgomeznt/Certificados/blob/master/guia/cacentos7.rst


Crear la Private Key y generar el Request
+++++++++++++++++++++++++++++++++++++++++++++++

Se debe generar un Private Key con el Request, esto se hace en la Entidad Certificadora::

	# openssl req -newkey rsa:2048 -nodes -keyout keyservice/srvutils.key -out request/srvutils.csr -subj "/C=VE/ST=DC/L=Caracas/O=PERSONAL/OU=TI/CN=srvutils"

	Generating a 2048 bit RSA private key
	.........................................+++
	..........+++
	writing new private key to 'keyservice/srvutils.key'
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

Creamos este archivo para tener una administración mas amplia, esto se hace en la Entidad Certificadora::

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

A continuación firmamos el pedido de certificado para generar el certificado para el servidor o servicio, esto se hace en la Entidad Certificadora::


	# openssl x509 -req -days 185 -extfile conf/srvutils.conf -extensions v3_req -CA certs/CA_empresa.crt -CAkey private/CA_empresa.key -CAserial ca.srl -CAcreateserial -in request/srvutils.csr -out newcerts/srvutils.crt
	Signature ok
	subject=/C=VE/ST=DC/L=Caracas/O=PERSONAL/OU=TI/CN=srvutils
	Getting CA Private Key
	Enter pass phrase for private/CA_empresa.key: Venezuela21




Si se coloca la opción "-policy policy_anything" indica que no se requiere que los campos "Country", "State" o "City", es para que coincidan con los de la CA.

Al finalizar se crean dos nuevos archivos:

* certs/srvutils.crt: Certificado del servidor, que puede hacerse públicamente disponible.

En este momento podemos eliminar el Request del certificado, el cual no necesitaremos más (srvutils.csr)::

	# rm –f request/srvutils.csr 


Preparar el directorio de trabajo
++++++++++++++++++++++++++++

Copiamos la llave y el certificado, también el certificado publico de la CA al directorio de trabajo, esto ya es en el servidor en donde estará el Application Server.::

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
	    localKeyID: 29 84 E7 1F 2E 6A D3 C6 B8 B7 CC C1 CB FD 35 F0 2A E9 C1 D5 
	    friendlyName: srvutils-key
	subject=/C=VE/ST=DC/L=Caracas/O=PERSONAL/OU=TI/CN=srvutils
	issuer=/C=VE/ST=DC/L=CCS/O=Default Company Ltd/OU=Sop App/CN=PERSONAL/emailAddress=root@personal.local
	-----BEGIN CERTIFICATE-----
	MIIDvjCCAqagAwIBAgIJALU559uWUDLnMA0GCSqGSIb3DQEBCwUAMIGPMQswCQYD
	VQQGEwJWRTELMAkGA1UECAwCREMxDDAKBgNVBAcMA0NDUzEcMBoGA1UECgwTRGVm
	YXVsdCBDb21wYW55IEx0ZDEQMA4GA1UECwwHU29wIEFwcDERMA8GA1UEAwwIUEVS
	U09OQUwxIjAgBgkqhkiG9w0BCQEWE3Jvb3RAcGVyc29uYWwubG9jYWwwHhcNMjEw
	ODI5MTUzMjQ0WhcNMjIwMzAyMTUzMjQ0WjBfMQswCQYDVQQGEwJWRTELMAkGA1UE
	CAwCREMxEDAOBgNVBAcMB0NhcmFjYXMxETAPBgNVBAoMCFBFUlNPTkFMMQswCQYD
	VQQLDAJUSTERMA8GA1UEAwwIc3J2dXRpbHMwggEiMA0GCSqGSIb3DQEBAQUAA4IB
	DwAwggEKAoIBAQDGPWqQKtN7MeMtVnj/mFioWCuq+HMoWKhop+KfD87kfNu7dTiK
	3ah9Bq/G6YgHGwXViWXOBRQRM6glCRU6J5a2E4p0H59xWg73DQ8oUvya8dn9ZU8R
	YE6EBKmuTTus0KktPLh3Ig4tu81Xtlk1JdZtNyAO1E7F5c8mxjAsCw0GaP18IZK3
	J7Rddr7n7G63rh2EjNQ8/R3tkwfgCZ7GOYlLyNXXQPMtZVsb15fY4xnyHUbni9hE
	pLq4nLFAUrGNwgbFQGy4hA691OvqDbrEXVDmpwlp02Wk7E3rGIcPfORYDdobRJlH
	FAeMICi0Gh0VhZuaxi26yWc4e48aZiOKA5qrAgMBAAGjTDBKMAkGA1UdEwQCMAAw
	CwYDVR0PBAQDAgXgMDAGA1UdEQQpMCeCDnNydnV0aWxzLmxvY2Fsgg9tb25pdG9y
	ZW8ubG9jYWyHBMCoABQwDQYJKoZIhvcNAQELBQADggEBALsgZY3KBFCn3Vw/Qnh0
	VEoVsPMBpAwBYgJR6rEVZffOmZJtnFxmBxFQt7F3ZBnNU4IqVzk7WyruVl8F78Ng
	aeIzvTsRXgZE5YfTvDFBgcd7O3pg4m82iB/xNQeaJlSs8f90EHV8dbe5yM3xuoVa
	jCiwtVvPtvMALRZdhLHIihp+6MQ0CbFmzU6acDsoM0GGtmP/TOytUdsexuMkABB2
	3CY5K8v9MfTuMn1XeGi4t+KXium5WQJL7oeWn44QW40IPqvPOSbTrR0cyP82acau
	CO4jZtHbd5nSVmyKmvKjD0PMn0QmhcC2W95hwI5C+7xW60aarx999LdZ1qoS04Au
	gBQ=
	-----END CERTIFICATE-----
	PKCS7 Data
	Shrouded Keybag: pbeWithSHA1And3-KeyTripleDES-CBC, Iteration 2048
	Bag Attributes
	    localKeyID: 29 84 E7 1F 2E 6A D3 C6 B8 B7 CC C1 CB FD 35 F0 2A E9 C1 D5 
	    friendlyName: srvutils-key
	Key Attributes: <No Attributes>
	Enter PEM pass phrase: changeit
	Verifying - Enter PEM pass phrase: changeit
	-----BEGIN ENCRYPTED PRIVATE KEY-----
	MIIFDjBABgkqhkiG9w0BBQ0wMzAbBgkqhkiG9w0BBQwwDgQIBFHn0NVAzvoCAggA
	MBQGCCqGSIb3DQMHBAjLBZ9S+ZK2sASCBMi5xScR00pY3N6HyQ2zGCq3i/vhmU/x
	gudTPJdqQRHbs/3/y5Gt/O7QlRK6nFtWctY8bqTD0SokRr9hAak9yR/yKedMGGJn
	zw2kw3BGrGB+/2IvV4AGl2Oy7gqekkXjG7OwoZnqh9HWFoI64qt4gkft7m7C6AFU
	nCKFo5DxFRY0mVEcN12s3cWieMMSUrfqbu20omGsAzyXgJYvT5opxKrtSGytroD6
	Is/DalFe6ra7sfLpRGHB45yrN5Z1p121NBZQnTJYtfxSXDAF6DcWE4f6iJdo+8sc
	4Io/GVRcP2sjkGPqT7uXKgSToe+c4hKv83XpvZcy46CDvYqgT8CvBAbT1X/YX4pH
	JaSoWl5Xgx3AR+fzoXo9mu8dsBgNtUTH0x2sHXuf3rPd0xPchoNHaqvOebOZ7Fjp
	lYmVEzn4VQf/3ekdiLJJTWu0mutRYHBBGMrGCRjIeMHyqod8x9YDgqEOqtMcEdxu
	whXIxf3stAWWvkfQFzQn4ntwFTkgyC4GKW2sSu9MK+rdv98vZt7OlKjtxpvT40XH
	YCMrS56C87a7TTJ44G4t/ZFR32UqB2bVrKV40HjOZmJuLOQuQmWsskXG6wMT7ZCP
	cgkXhSAchFFCTWQZNIzpkNhbkg6Ynw9iSY1fc0q38hdaJLLbX8bGhkAP7PeZgycH
	p0NxroUeqQ3G4isvP4ufoILukmTMADldq8i5xCd0706bn5Ks5ya9viqstO9ZvPuz
	TjfEngv495riINSknBjzBdq84jBV2Qdlvk3hlMJ17ZQOs8HVC4LyzuHOm1j/nLsO
	bVjhdouVB7VrLyZTYf1XzxmRsscTD0Q4EkaR9NwDjW2Ea0y1VVRbR92KDdk83B63
	RbfSXToUpaOTmYxc4zAVOUYYu72zeGm9RtTUE2IqSlRr0kGqrxOIda/ljNmJpsHv
	RS1bm/FaP/PCfd+kxDHHLaSUOxq2qWGhSxEds5In1jpsXNdOQ7gcS7t91TyyoOvZ
	RLNqxxc9nDWeCmF/RsXPWckehm4KFJdaI+DAKyByBOKGmOwo6GEHVJ7g53gPgknF
	WFGfGF9OHapKD5sOyMzF5jYmH/Q4Tq+LC4THlrEJOXTy/MgQq1Ve//7UWkzC0DUO
	NHZrFLXScHqhoNRzRacZ0P9YtETtnQqkkkkPE8iOxdD3ZHqQy6gE1ngAe7k/TiH+
	+hqMYYnINXtAtKFipgdzVEl8KEg/DQtqCPYYEu3VPW1mlBUWouOOb997EKB3IYSR
	pmFfu2x06LyxCQKFTRA/olNfTnES+z/1PbIlfeyMtvYF13fu5tkB3LH7i/XTOlH6
	7Xgb1ljDg322eNyAe7iJPqllov1hX0w1n9Es5a9Zw8W2L5OLX0wb9jD/qrTHQDiD
	2t/6puPL5Am3FVeD+iRJFU3nfKqkQuzETsbnmjUIFiXQ4F0grrRte725m5BcSMbh
	wQjDTNm0J6OphQArJKOC1U2IPqkzfql/26TJN5moycs6ctbfo4RRiZvS8T5o1Fcv
	MGz+LRmNEtN43m6+B6im03q7ccAArI1DjAcktFJxKscts0bL3sPj9BULJyJsVrQk
	tAmg1dpwhiKKVfyzkT3Q6Mqd3Lk9jRmaeFim8R4zbFV4ZRFIUjf2xqtA6gLWxgbR
	v48=
	-----END ENCRYPTED PRIVATE KEY-----
	# 

**IMPORTANTE** Recordar que cuando se crea el .p12 estamos ya incluyendo dentro de él, la Private Key y el certificado.


Crear el almacén de claves Keystore 
++++++++++++++++++++++++++

Creamos el almacén de claves Keystore agregando el p12 anteriormente::

	# keytool -importkeystore -destkeystore keystore.jks -srckeystore srvutils.p12 -srcstoretype pkcs12 -alias srvutils-key

	Introduzca la contraseña de almacén de claves de destino:  changeit
	Volver a escribir la contraseña nueva: changeit
	Introduzca la contraseña de almacén de claves de origen:  changeit
	# 

**IMPORTANTE** Si no utilizo las mismas claves para el Java KeyStore y de la Private Key, no continué, no va funcionar.

Consultamos el keystore y debemos ver las entrada de la private key::

	# keytool -list -v -keystore keystore.jks --storepass changeit

	Tipo de Almacén de Claves: JKS
	Proveedor de Almacén de Claves: SUN

	Su almacén de claves contiene 1 entrada

	Nombre de Alias: srvutils-key
	Fecha de Creación: 29/08/2021
	Tipo de Entrada: PrivateKeyEntry
	Longitud de la Cadena de Certificado: 1
	Certificado[1]:
	Propietario: CN=srvutils, OU=TI, O=PERSONAL, L=Caracas, ST=DC, C=VE
	Emisor: EMAILADDRESS=root@personal.local, CN=PERSONAL, OU=Sop App, O=Default Company Ltd, L=CCS, ST=DC, C=VE
	Número de serie: b539e7db965032e7
	Válido desde: Sun Aug 29 11:32:44 EDT 2021 hasta: Wed Mar 02 10:32:44 EST 2022
	Huellas digitales del Certificado:
		 MD5: 3F:6C:FE:D6:59:C5:25:AA:E0:3B:42:1F:2E:C1:E6:C3
		 SHA1: 29:84:E7:1F:2E:6A:D3:C6:B8:B7:CC:C1:CB:FD:35:F0:2A:E9:C1:D5
		 SHA256: BA:46:55:0B:3A:56:47:61:57:40:3E:02:E4:B7:27:CD:A5:71:77:58:A8:C3:00:6A:53:C6:F1:56:89:AB:DE:72
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


Importar la CA dentro del keystore
++++++++++++++++++++++++++++++++++++

Agregar el certificado publico de la CA dentro del keystore::

	# keytool -import -trustcacerts -alias ca-certificate -file CA_empresa.crt -keystore keystore.jks -storepass changeit

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
	# 



Consultamos el keystore y debemos ver las entrada de la private key y el certificado publico de la CA::

	# keytool -list -v -keystore keystore.jks -storepass changeit

	Tipo de Almacén de Claves: JKS
	Proveedor de Almacén de Claves: SUN

	Su almacén de claves contiene 2 entradas

	Nombre de Alias: ca-certificate
	Fecha de Creación: 29/08/2021
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


	Nombre de Alias: srvutils-key
	Fecha de Creación: 29/08/2021
	Tipo de Entrada: PrivateKeyEntry
	Longitud de la Cadena de Certificado: 1
	Certificado[1]:
	Propietario: CN=srvutils, OU=TI, O=PERSONAL, L=Caracas, ST=DC, C=VE
	Emisor: EMAILADDRESS=root@personal.local, CN=PERSONAL, OU=Sop App, O=Default Company Ltd, L=CCS, ST=DC, C=VE
	Número de serie: b539e7db965032e7
	Válido desde: Sun Aug 29 11:32:44 EDT 2021 hasta: Wed Mar 02 10:32:44 EST 2022
	Huellas digitales del Certificado:
		 MD5: 3F:6C:FE:D6:59:C5:25:AA:E0:3B:42:1F:2E:C1:E6:C3
		 SHA1: 29:84:E7:1F:2E:6A:D3:C6:B8:B7:CC:C1:CB:FD:35:F0:2A:E9:C1:D5
		 SHA256: BA:46:55:0B:3A:56:47:61:57:40:3E:02:E4:B7:27:CD:A5:71:77:58:A8:C3:00:6A:53:C6:F1:56:89:AB:DE:72
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
	# 




Publicar el Java KeyStore
++++++++++++++++++++++++

El Java KeyStore creado ahora debe ser entregado y publicado dentro del **Application Server** (Tomcat, JBoss, Glasfish, Weblogic, WebSphere, etc.)

Puede leer los siguientes link para hacer la publicación en un JBoss o Tomcat.

https://github.com/cgomeznt/JBOSS/blob/master/guia/ssl7x.rst

https://github.com/cgomeznt/Tomcat/blob/master/guia/ssl.rst



Cómo aceptar certificados en el navegador
+++++++++++++++++++++++++++++++++++++++


Un application server que en su sitio web utiliza SSL, proporciona a los navegadores un certificado que certifica su identidad. Este certificado contiene información, el cual el navegador confirma su validez por medio de los certificados root server de confianza que tiene en su repositorio de certificados. Al comprobar que el certificado es valido, se confirma que has accedido al sitio de forma segura y ha establecido una conexión SSL.

En resumen, del lado de los navegadores siempre se debe tener en el repositorio de certificados root server de confianza, el certificado publico de la CA que firmo el certificado que esta expuesto en el Application Server.

Para instalar el certificado de la CA en CentOS 7::

	# cp CA_certificado.crt /etc/pki/ca-trust/source/anchors/
	# update-ca-trust

Para instalar en el certificado de la CA en Windows::

	win+r certmgr.msc
	En el marco izquierdo, expanda Entidades de Certificacion raíz de confianza, haga clic con el botón derecho en Certificados y seleccione Todas las tareas >Importar, selecciones CA_certificado.crt

Con lo anterior el Internet Explorer y Google utilizaran dicha CA, pero Firefox NO.

En Firefox hay que ir a Herramientas -> Opciones -> Avanzado -> Certificados -> Ver certificados -> Importar y una vez allí importar el archivo CA_certificado.crt

Para demostrar lo antes dicho hagamos lo siguiente, con un navegador preferiblemente FIREFOX lo abrimos y vamos hasta la URL de nuestro sitio Web, podremos ver que es un lugar inseguro. 

.. figure:: ../images/keystore/01.png


Abrimos el certificado y vemos quien lo firmo.


.. figure:: ../images/keystore/02.png

Nos vamos al botón ver certificado para tener más detalle del certificado publico de la CA que necesitamos.

.. figure:: ../images/keystore/03.png

Ya cuando tengamos el detalle y sabemos cual es el certificado publico de la CA, buscamos cualquier técnica y descargamos el certificado. Cuando lo tengamos le vamos a indicar al FIREFOX que lo importe en su repositorio de certificados de confianza. Y para hacerlo nos vamos a Preferencias, en Privacidad y Seguridad, buscamos certificados.


.. figure:: ../images/keystore/04.png



Luego le damos Ver certificados y buscamos el botón importar

.. figure:: ../images/keystore/05.png


Buscamos el certificado publico de la CA, lo seleccionamos y le damos aceptar y listo ya estará en el repositorio de certificados de confianza de FIREFOX

.. figure:: ../images/keystore/06.png

Seleccionamos el botón de Editar Confianza y marcamos los dos (2) check y listo ya tenemos la configuración requerida.


.. figure:: ../images/keystore/07.png


Nos vamos nuevamente a la URL y vemos que ahora si se produce de forma exitosa el HandShake y el navegador ve seguro el certificado.

.. figure:: ../images/keystore/08.png





En el siguiente link explica como importar dentro de un servidor el certificado publico de la CA.

https://github.com/cgomeznt/Certificados/blob/master/guia/addcertificaterootserver.rst

Este otro link puede ser de interés.

https://github.com/cgomeznt/Certificados/blob/master/guia/NavegadoresTrustedRoot.rst

Este otro link explica como hacer un troubleshooting y aclara los fundamentos.

https://github.com/cgomeznt/Certificados/blob/master/guia/openssl_verificar-URL_su_certificado_publico_CA.rst



**NOTA** En caso de emergencia si requerimos cambiar el password, si al private key le colocaron una clave distinta a la del KeyStore, por ejemplo Venezuela21, podemos modificar esta clave y colocar una igual al KeyStore.

Como saber si deben utilizarla, si ven un error como este::

	Caused by: java.security.UnrecoverableKeyException: Cannot recover key

Entonces para solventar seria::

	keytool -keypasswd -new changeit -keystore keystore.jks -storepass changeit -alias srvutils-key -keypass Venezuela21

