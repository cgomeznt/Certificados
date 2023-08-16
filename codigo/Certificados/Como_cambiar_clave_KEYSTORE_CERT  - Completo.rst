Como cambiar la clave del KEYSTORE y del certificado
========================================================

En este ejemplo vamos a utilizar el KEYSTORE de la Intranet de QA que es::

	lcsqaappintranet.credicard.com.ve.jks

Este certificado es utilizado por el Tomcat de Calidad::

     <Connector port="8443" protocol="org.apache.coyote.http11.Http11Nio2Protocol"
               maxThreads="150" SSLEnabled="true">
        <SSLHostConfig protocols="TLSv1,TLSv1.1,TLSv1.2"
                ciphers="TLS_RSA_WITH_AES_128_CBC_SHA, TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA256,
                TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA, TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA384,
                TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA, TLS_RSA_WITH_AES_128_CBC_SHA256,
                TLS_DHE_RSA_WITH_AES_128_CBC_SHA, TLS_DHE_DSS_WITH_AES_128_CBC_SHA,
                SSL_RSA_WITH_3DES_EDE_CBC_SHA, SSL_DHE_RSA_WITH_3DES_EDE_CBC_SHA,
                SSL_DHE_DSS_WITH_3DES_EDE_CBC_SHA" >
            <Certificate certificateKeystoreFile="/opt/tomcat/conf/lcsqaappintranet.credicard.com.ve.jks" certificateKeystorePassword="Zxe+P.3d67.x7Tp" certificateKeystoreType="JKS"  type="RSA" />
        </SSLHostConfig>
    </Connector>


El certificado lo copiamos de /opt/tomcat/conf/lcsqaappintranet.credicard.com.ve.jks /home/cgomez/
	
Consultamos el certificado primeramente::

	# /opt/jdk-12.0.2/bin/keytool -list -v -storepass 'Zxe+P.3d67.x7Tp' -keystore /home/cgomez/lcsqaappintranet.credicard.com.ve.jks
	
		Keystore type: JKS
		Keystore provider: SUN

		Your keystore contains 2 entries

		Alias name: ca credicard
		Creation date: Jan 11, 2022
		Entry type: trustedCertEntry

		Owner: CN=Consorcio Credicard CA, OU=Criptografia, O=Consorcio Credicard CA, L=Caracas, ST=Distrito Capital, C=VE
		Issuer: CN=Consorcio Credicard CA, OU=Criptografia, O=Consorcio Credicard CA, L=Caracas, ST=Distrito Capital, C=VE
		Serial number: c047150eb9d89fc7
		Valid from: Tue Jul 27 15:46:01 VET 2021 until: Sun Jul 26 15:46:01 VET 2026
		Certificate fingerprints:
				 SHA1: 74:02:5E:0C:99:22:D9:BD:A2:47:DE:A2:11:21:5D:31:7C:29:2A:51
				 SHA256: 2D:FB:EA:E0:86:A3:F2:FF:E3:0C:E9:92:DF:DB:E0:C7:16:86:14:44:32:80:28:3A:29:EF:0A:8A:E5:5A:B4:42
		Signature algorithm name: SHA256withRSA
		Subject Public Key Algorithm: 2048-bit RSA key
		Version: 3

		Extensions:

		#1: ObjectId: 2.5.29.35 Criticality=false
		AuthorityKeyIdentifier [
		KeyIdentifier [
		0000: 9C 1B A9 19 17 18 64 22   E1 38 63 72 2B 61 1B 72  ......d".8cr+a.r
		0010: 9F 70 22 43                                        .p"C
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
		0000: 9C 1B A9 19 17 18 64 22   E1 38 63 72 2B 61 1B 72  ......d".8cr+a.r
		0010: 9F 70 22 43                                        .p"C
		]
		]



		*******************************************
		*******************************************


		Alias name: 2101122747862202778cn=lcsqaappintranet.credicard.com.ve, ou=criptografia, o=consorcio credicard c.a, l=caracas, st=distrito capital, c=vecn=consorcio credicard ca, ou=criptografia, o=consorcio credicard ca, l=caracas, st=distrito capital, c=ve
		Creation date: Jan 11, 2022
		Entry type: PrivateKeyEntry
		Certificate chain length: 2
		Certificate[1]:
		Owner: CN=lcsqaappintranet.credicard.com.ve, OU=Criptografia, O=Consorcio Credicard C.A, L=Caracas, ST=Distrito Capital, C=VE
		Issuer: CN=Consorcio Credicard CA, OU=Criptografia, O=Consorcio Credicard CA, L=Caracas, ST=Distrito Capital, C=VE
		Serial number: 1d28b001c7d1b99a
		Valid from: Tue Jan 11 15:34:00 VET 2022 until: Wed Jan 11 15:34:00 VET 2023
		Certificate fingerprints:
				 SHA1: 5D:ED:E8:AE:C3:D2:73:BB:88:B8:4D:D1:19:28:61:9C:62:C2:B7:5C
				 SHA256: B6:86:D9:B3:47:B1:61:D2:DD:B9:58:99:22:1B:09:9C:CF:DC:95:C8:C0:55:42:8F:6A:42:46:6F:C7:38:AF:15
		Signature algorithm name: SHA256withRSA
		Subject Public Key Algorithm: 2048-bit RSA key
		Version: 3

		Extensions:

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
		  DNSName: lcsqaappintranet.credicard.com.ve
		  DNSName: intragestionqa.credicard.com.ve
		  DNSName: intragestionqa
		  DNSName: lcsqaappintranet
		  IPAddress: 10.134.0.81
		]

		Certificate[2]:
		Owner: CN=Consorcio Credicard CA, OU=Criptografia, O=Consorcio Credicard CA, L=Caracas, ST=Distrito Capital, C=VE
		Issuer: CN=Consorcio Credicard CA, OU=Criptografia, O=Consorcio Credicard CA, L=Caracas, ST=Distrito Capital, C=VE
		Serial number: c047150eb9d89fc7
		Valid from: Tue Jul 27 15:46:01 VET 2021 until: Sun Jul 26 15:46:01 VET 2026
		Certificate fingerprints:
				 SHA1: 74:02:5E:0C:99:22:D9:BD:A2:47:DE:A2:11:21:5D:31:7C:29:2A:51
				 SHA256: 2D:FB:EA:E0:86:A3:F2:FF:E3:0C:E9:92:DF:DB:E0:C7:16:86:14:44:32:80:28:3A:29:EF:0A:8A:E5:5A:B4:42
		Signature algorithm name: SHA256withRSA
		Subject Public Key Algorithm: 2048-bit RSA key
		Version: 3

		Extensions:

		#1: ObjectId: 2.5.29.35 Criticality=false
		AuthorityKeyIdentifier [
		KeyIdentifier [
		0000: 9C 1B A9 19 17 18 64 22   E1 38 63 72 2B 61 1B 72  ......d".8cr+a.r
		0010: 9F 70 22 43                                        .p"C
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
		0000: 9C 1B A9 19 17 18 64 22   E1 38 63 72 2B 61 1B 72  ......d".8cr+a.r
		0010: 9F 70 22 43                                        .p"C
		]
		]



		*******************************************
		*******************************************



		Warning:
		The JKS keystore uses a proprietary format. It is recommended to migrate to PKCS12 which is an industry standard format using "keytool -importkeystore -srckeystore /opt/tomcat/conf/lcsqaappintranet.credicard.com.ve.jks -destkeystore /opt/tomcat/conf/lcsqaappintranet.credicard.com.ve.jks -deststoretype pkcs12".
		#



Cambiamos la clave del KEYSTORE::

	# /opt/jdk-12.0.2/bin/keytool -storepasswd -new 'NUEVA_CLAVE' -keystore /home/cgomez/lcsqaappintranet.credicard.com.ve.jks -storepass 'Zxe+P.3d67.x7Tp'

		Warning:
		The JKS keystore uses a proprietary format. It is recommended to migrate to PKCS12 which is an industry standard format using "keytool -importkeystore -srckeystore /home/cgomez/lcsqaappintranet.credicard.com.ve.jks -destkeystore /home/cgomez/lcsqaappintranet.credicard.com.ve.jks -deststoretype pkcs12".
		[root@lcsqaappintranet cgomez]#


Cambiamos la clave del certificado (debe ser igual al KEYSTORE)::

	# /opt/jdk-12.0.2/bin/keytool -keypasswd -alias '2101122747862202778cn=lcsqaappintranet.credicard.com.ve, ou=criptografia, o=consorcio credicard c.a, l=caracas, st=distrito capital, c=vecn=consorcio credicard ca, ou=criptografia, o=consorcio credicard ca, l=caracas, st=distrito capital, c=ve' -keypass 'Zxe+P.3d67.x7Tp' -new 'NUEVA_CLAVE' -keystore /home/cgomez/lcsqaappintranet.credicard.com.ve.jks -storepass 'NUEVA_CLAVE'

		Warning:
		The JKS keystore uses a proprietary format. It is recommended to migrate to PKCS12 which is an industry standard format using "keytool -importkeystore -srckeystore /home/cgomez/lcsqaappintranet.credicard.com.ve.jks -destkeystore /home/cgomez/lcsqaappintranet.credicard.com.ve.jks -deststoretype pkcs12".
		[root@lcsqaappintranet cgomez]#

		
Certificamos que todo este bien
---------------------------------

Si todo ha marchado bien debe poder realizar ::

	# /opt/jdk-12.0.2/bin/keytool -importkeystore -srckeystore /home/cgomez/lcsqaappintranet.credicard.com.ve.jks -destkeystore /home/cgomez/lcsqaappintranet.credicard.com.ve.jks -deststoretype pkcs12                      Enter source keystore password:

		Entry for alias ca credicard successfully imported.
		Entry for alias 2101122747862202778cn=lcsqaappintranet.credicard.com.ve, ou=criptografia, o=consorcio credicard c.a, l=caracas, st=distrito capital, c=vecn=consorcio credicard ca, ou=criptografia, o=consorcio credicard ca, l=caracas, st=distrito capital, c=ve successfully imported.
		Import command completed:  2 entries successfully imported, 0 entries failed or cancelled

		Warning:
		Migrated "/home/cgomez/lcsqaappintranet.credicard.com.ve.jks" to PKCS12. The JKS keystore is backed up as "/home/cgomez/lcsqaappintranet.credicard.com.ve.jks.old".

Si algo salio mal, tendra un erro como este::
			
	# /opt/jdk-12.0.2/bin/keytool -importkeystore -srckeystore /home/cgomez/lcsqaappintranet.credicard.com.ve.jks -destkeystore /home/cgomez/lcsqaappintranet.credicard.com.ve.jks -deststoretype pkcs12
	
		Enter source keystore password:NUEVA_CLAVE
		Entry for alias ca credicard successfully imported.
		Enter key password for <2101122747862202778cn=lcsqaappintranet.credicard.com.ve, ou=criptografia, o=consorcio credicard c.a, l=caracas, st=distrito capital, c=vecn=consorcio credicard ca, ou=criptografia, o=consorcio credicard ca, l=caracas, st=distrito capital, c=ve>:NUEVA_CLAVE
		keytool error: java.security.UnrecoverableKeyException: Cannot recover key


