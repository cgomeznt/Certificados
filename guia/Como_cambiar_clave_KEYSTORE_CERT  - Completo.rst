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
	
Consultamos el certificado primeramente;;

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

Si todo ha marchado bien debe poder realizar 
	# /opt/jdk-12.0.2/bin/keytool -importkeystore -srckeystore /home/cgomez/lcsqaappintranet.credicard.com.ve.jks -destkeystore /home/cgomez/lcsqaappintranet.credicard.com.ve.jks -deststoretype pkcs12                      Enter source keystore password:

		Entry for alias ca credicard successfully imported.
		Entry for alias 2101122747862202778cn=lcsqaappintranet.credicard.com.ve, ou=criptografia, o=consorcio credicard c.a, l=caracas, st=distrito capital, c=vecn=consorcio credicard ca, ou=criptografia, o=consorcio credicard ca, l=caracas, st=distrito capital, c=ve successfully imported.
		Import command completed:  2 entries successfully imported, 0 entries failed or cancelled

		Warning:
		Migrated "/home/cgomez/lcsqaappintranet.credicard.com.ve.jks" to PKCS12. The JKS keystore is backed up as "/home/cgomez/lcsqaappintranet.credicard.com.ve.jks.old".

			
	# /opt/jdk-12.0.2/bin/keytool -importkeystore -srckeystore /home/cgomez/lcsqaappintranet.credicard.com.ve.jks -destkeystore /home/cgomez/lcsqaappintranet.credicard.com.ve.jks -deststoretype pkcs12
	
		Enter source keystore password:NUEVA_CLAVE
		Entry for alias ca credicard successfully imported.
		Enter key password for <2101122747862202778cn=lcsqaappintranet.credicard.com.ve, ou=criptografia, o=consorcio credicard c.a, l=caracas, st=distrito capital, c=vecn=consorcio credicard ca, ou=criptografia, o=consorcio credicard ca, l=caracas, st=distrito capital, c=ve>:NUEVA_CLAVE
		keytool error: java.security.UnrecoverableKeyException: Cannot recover key




Los siguientes pasos es para certificar
------------------------------------------


Exportamos el certificado y el key::

	# /opt/jdk-12.0.2/bin/keytool -importkeystore \
	     -srckeystore /home/cgomez/lcsqaappintranet.credicard.com.ve.jks \
	     -destkeystore keystore.p12 \
	     -deststoretype PKCS12 \
	     -srcalias '2101122747862202778cn=lcsqaappintranet.credicard.com.ve, ou=criptografia, o=consorcio credicard c.a, l=caracas, st=distrito capital, c=vecn=consorcio credicard ca, ou=criptografia, o=consorcio credicard ca, l=caracas, st=distrito capital, c=ve' \
	     -deststorepass r00tme \
	     -destkeypass r00tme
		Importing keystore /home/cgomez/lcsqaappintranet.credicard.com.ve.jks to keystore.p12...
		Enter source keystore password:NUEVA_CLAVE
		[root@lcsqaappintranet cgomez]#

	
Consultamos el p12::

	# openssl pkcs12 -info -in keystore.p12
	
		Enter Import Password:r00tme
		MAC Iteration 100000
		MAC verified OK
		PKCS7 Data
		Shrouded Keybag: pbeWithSHA1And3-KeyTripleDES-CBC, Iteration 50000
		Bag Attributes
			friendlyName: 2101122747862202778cn=lcsqaappintranet.credicard.com.ve, ou=criptografia, o=consorcio credicard c.a, l=caracas, st=distrito capital, c=vecn=consorcio credicard ca, ou=criptografia, o=consorcio credicard ca, l=caracas, st=distrito capital, c=ve
			localKeyID: 54 69 6D 65 20 31 36 38 34 38 36 39 33 34 39 30 34 39
		Key Attributes: <No Attributes>
		Enter PEM pass phrase:r00tme
		Verifying - Enter PEM pass phrase:r00tme
		-----BEGIN ENCRYPTED PRIVATE KEY-----
		MIIFDjBABgkqhkiG9w0BBQ0wMzAbBgkqhkiG9w0BBQwwDgQICCT34LRMF7ECAggA
		MBQGCCqGSIb3DQMHBAgRHSZ2gWzs8wSCBMjAThaZmSC8q905mK/OZC7hQPSKpYBC
		VmO0wj2ReuCBtqnVW/KuQnKJLUTyurH8893mL8TbtIMHMZZmcrsWLHEYBwtApfqu
		CS8A97FJ0Ka9ErFbHASRH00t2pnNgU4fkVC/tgao+qNb+sV2py39hCQxlrAvCMyM
		OORvfUw4Uqd6QC1ZaewQlkFpsjMN7GZ304YpTk1TIktrQKSjiRkFmmDQSBI88nm6
		IXmXYdrKkr4cr5QCZH1wa/8LS3vFvHkZsFQrv3w9tyB4WS3jbsYclq3y/CxX1BUK
		04SysQY1MlWf9oYyCCv7ar9yFb8s4mC9HlHmQiJlIymBoWxtg8pVhcOXp96awsy3
		0bE8io2m10AvIrpG+8k/N3jgkveJj51Q+pdW3tSTRqetZyFZDmoIYOPNlWr6qDQj
		wnpFAf6Qw/tiRTXc6inDPzQTc9mx5sYlOxouDdohF130+70JcRG0IQh/jCqw3LPR
		Pgxa4Px2ZZJoxbOJ7dFUfkyodcIu80StVHULaHxkWEQhYfM90FkoxYK0bIgJqWRI
		EqN5XKC/JdOkVdHI3cVPCPx3Grkx6K9nonKIqXCINCAdz0sRLQ4vDuCM54Bw+p18
		1hGsGUHFVs5YcoKKe8DptYAyy2CC+76052prv0hud4YkS4ZaT7aHmZvd68M7yr+L
		ImHMiBZyhU56r/qSLm4A3ylIUt5d1BIkaGOqE8WM9BcECIQ4kKasUJ8x6LLhOU+A
		ekzB2rBFqy6pYkRsN8vqeEavqyEZw6L3Ci8hBVcBD5/cLqYKg/u8sEDKcL1OA9kg
		TmtNvSEazj70cgKtj3PcYC66Q1T0vMA67GnHpC6mCZyPUArHCh+m4iTrfprirAFt
		r1pPKBvocAp0UI7IvKC/ntsKyzsHL5rEvyqwxIn08gC55JFMwtCYxD71O2kFcEnh
		7V8oH436ea3bozh2ovQmiI1XoKokqccL8pqpybl9zA35/ea3nLSLxn09AH9yCnJR
		OkVdg6MmSTCnK8yTJJIY1nE76vp9hU3TwqWXIlfeEz3QUU7vgwXH0Jhnz3pkLgH8
		pPgBNdUdObyiC+XjbcDjfURt0HPf6lDjTdp3M3ckaSumm4w/9Km8XJoEZgk2ni8w
		AA1bkdxJcRj5EMFLFO+OIvkKcBHNSB3PYPwaPXurIvh1WPTvQCJf/WJu/8y6I7qM
		yqwNAKu90Wic5C3DhdS3/qdhIC32EC8rZEiQhv8bJLNr3MUK9lVGd1pfTIw8wcxV
		3IHReWJOKqH/fIiIZbNOY4Mk83ryUZxpOtjBDY0NcN1f/CkORnomDsMdZvWRt/wS
		Axcx9gdBb2btXcx/zn95nn5xYWs7JAaobR2etYNWOt+K1sfD76q3oixeOA0tpUkn
		/nrN31y6aEZOWLuBjN7B2X1mwwJb653LF3sr+EthK3IffAZwC9ZQtu4msuY/4O3x
		qk8Wy1jzxfe46BgGvYzvl+uPzI45tjFUN5CDNovwfh3lVJaKoeU2eQFeo6/vUL/j
		5a2DWsEmGRDEbCLeicoMHQ2qBpgFI/GWMaM9KhtHwO65AjZOkzQtT7MnY+TblqC/
		ajjaQQW+d6axU6pVgywy7VePMnBPm8XW1Y9y67SCsWvezRFgPi49GXNw1dvUvVDa
		kB4=
		-----END ENCRYPTED PRIVATE KEY-----
		PKCS7 Encrypted data: pbeWithSHA1And40BitRC2-CBC, Iteration 50000
		Certificate bag
		Bag Attributes
			friendlyName: 2101122747862202778cn=lcsqaappintranet.credicard.com.ve, ou=criptografia, o=consorcio credicard c.a, l=caracas, st=distrito capital, c=vecn=consorcio credicard ca, ou=criptografia, o=consorcio credicard ca, l=caracas, st=distrito capital, c=ve
			localKeyID: 54 69 6D 65 20 31 36 38 34 38 36 39 33 34 39 30 34 39
		subject=/C=VE/ST=Distrito Capital/L=Caracas/O=Consorcio Credicard C.A/OU=Criptografia/CN=lcsqaappintranet.credicard.com.ve
		issuer=/C=VE/ST=Distrito Capital/L=Caracas/O=Consorcio Credicard CA/OU=Criptografia/CN=Consorcio Credicard CA
		-----BEGIN CERTIFICATE-----
		MIIESTCCAzGgAwIBAgIIHSiwAcfRuZowDQYJKoZIhvcNAQELBQAwgZMxCzAJBgNV
		BAYTAlZFMRkwFwYDVQQIDBBEaXN0cml0byBDYXBpdGFsMRAwDgYDVQQHDAdDYXJh
		Y2FzMR8wHQYDVQQKDBZDb25zb3JjaW8gQ3JlZGljYXJkIENBMRUwEwYDVQQLDAxD
		cmlwdG9ncmFmaWExHzAdBgNVBAMMFkNvbnNvcmNpbyBDcmVkaWNhcmQgQ0EwHhcN
		MjIwMTExMTkzNDAwWhcNMjMwMTExMTkzNDAwWjCBnzELMAkGA1UEBhMCVkUxGTAX
		BgNVBAgMEERpc3RyaXRvIENhcGl0YWwxEDAOBgNVBAcMB0NhcmFjYXMxIDAeBgNV
		BAoMF0NvbnNvcmNpbyBDcmVkaWNhcmQgQy5BMRUwEwYDVQQLDAxDcmlwdG9ncmFm
		aWExKjAoBgNVBAMMIWxjc3FhYXBwaW50cmFuZXQuY3JlZGljYXJkLmNvbS52ZTCC
		ASIwDQYJKoZIhvcNAQEBBQADggEPADCCAQoCggEBAL1abjsm2wkMghtoFuEcmxax
		Eh2oHU0X02VeeigJZxvLOU+VIR6gs67VSlvICVST02wYR5TDziisb64drM14TdZB
		hT+P8FRoc+lIGZNEFRDNM4gOF5Tf2ArB7PJSjuTEZKgjIvgTluhpSHomiQ9ivT+F
		iAAx6WS+t2EZy1cykVqNTu9jl5FrpPLJ0DoEACZVmeR+FCHTBcmbbrJ/PsOL1EF4
		jQ1EcqVTCwRn73purzbB83Fm/eCP3u6P2dQOjzH+oyWA8Tu0P0bL34dWdJksYaup
		tynL4nCp+NxWN+wnUrqTuWj3xcgxFAW8i7e0Po5v3B5xykrA60RmR8UgFqlE2jsC
		AwEAAaOBkjCBjzAJBgNVHRMEAjAAMAsGA1UdDwQEAwIF4DB1BgNVHREEbjBsgiFs
		Y3NxYWFwcGludHJhbmV0LmNyZWRpY2FyZC5jb20udmWCH2ludHJhZ2VzdGlvbnFh
		LmNyZWRpY2FyZC5jb20udmWCDmludHJhZ2VzdGlvbnFhghBsY3NxYWFwcGludHJh
		bmV0hwQKhgBRMA0GCSqGSIb3DQEBCwUAA4IBAQAZrAQUOtXWcU2V5+C60lYyCOmB
		NKsOLD3PhjOi0tPQfvETDnn8uNzmQPuynJs1Uaue0iPQfmMLuJPz5E7d3SCCFX+P
		BFbvR+O8/9DKPqNg+gWx+pOQQbowauNBufya4TDL2bbNGUhHLuzEYLb/JfkOxlrV
		SOuTb2HqiVn3JHKxBnikq8VxS0yk/6/nvt3B3dYqj2kf7oNHanjmw1rSMaP9V3hE
		D6L1HZviU9ftKg0ql3BJ9PpXwO21D4LZD8/fpYbmZYrB8M6ozw7w/Mvl+FGTAQky
		9bUL3NZnt5xPZENKezskeMYAnazG1zc7buk+6Im8FIDsNu7DzLYkN8gXpzJU
		-----END CERTIFICATE-----
		Certificate bag
		Bag Attributes
			friendlyName: CN=Consorcio Credicard CA,OU=Criptografia,O=Consorcio Credicard CA,L=Caracas,ST=Distrito Capital,C=VE
		subject=/C=VE/ST=Distrito Capital/L=Caracas/O=Consorcio Credicard CA/OU=Criptografia/CN=Consorcio Credicard CA
		issuer=/C=VE/ST=Distrito Capital/L=Caracas/O=Consorcio Credicard CA/OU=Criptografia/CN=Consorcio Credicard CA
		-----BEGIN CERTIFICATE-----
		MIID+zCCAuOgAwIBAgIJAMBHFQ652J/HMA0GCSqGSIb3DQEBCwUAMIGTMQswCQYD
		VQQGEwJWRTEZMBcGA1UECAwQRGlzdHJpdG8gQ2FwaXRhbDEQMA4GA1UEBwwHQ2Fy
		YWNhczEfMB0GA1UECgwWQ29uc29yY2lvIENyZWRpY2FyZCBDQTEVMBMGA1UECwwM
		Q3JpcHRvZ3JhZmlhMR8wHQYDVQQDDBZDb25zb3JjaW8gQ3JlZGljYXJkIENBMB4X
		DTIxMDcyNzE5NDYwMVoXDTI2MDcyNjE5NDYwMVowgZMxCzAJBgNVBAYTAlZFMRkw
		FwYDVQQIDBBEaXN0cml0byBDYXBpdGFsMRAwDgYDVQQHDAdDYXJhY2FzMR8wHQYD
		VQQKDBZDb25zb3JjaW8gQ3JlZGljYXJkIENBMRUwEwYDVQQLDAxDcmlwdG9ncmFm
		aWExHzAdBgNVBAMMFkNvbnNvcmNpbyBDcmVkaWNhcmQgQ0EwggEiMA0GCSqGSIb3
		DQEBAQUAA4IBDwAwggEKAoIBAQC9xoRfPl7HdF25Cex8SGmGoPAujFPp4BVYhD2V
		MwVQnkIClUzGRImtldIj9tQjDUplAEVmpA4AhH4pSo+hHEv/+KtjfYnrsAMf7PMk
		cs5nS4/LrKs7FgVqcI3xoHbco7GQ33tOvwe1LXUPXjCp3zyHkkEfUhJ3N6FXuIvZ
		B1D6Vk7qQKJjURxRCG9WlQ/kmM0qD4TdVDcF0D0vIQW81DBj+vaXkMBf471r6fCu
		vc1nd6jdQ6jqvB4Hp/MF4cf5QF9PPRhd9Xdj/d1f0TxNzLaIB2ZAtj/WNWypJ5qd
		AXVL4INpychzCek3FaKQWmZfyNPkATkFjh06eOccrkx571XHAgMBAAGjUDBOMB0G
		A1UdDgQWBBScG6kZFxhkIuE4Y3IrYRtyn3AiQzAfBgNVHSMEGDAWgBScG6kZFxhk
		IuE4Y3IrYRtyn3AiQzAMBgNVHRMEBTADAQH/MA0GCSqGSIb3DQEBCwUAA4IBAQCt
		8Ii8ZyPg1I3XOTa3Lch2z8+eYQBAcvedNStBBNfkKXiydmiAxF8sqdR7IZaBGIPP
		UyP+E4zgKoIcbopLbjJehxwyaiKAIscr1GW71HoDZqwS8kWGB6EootN1SvIFpAXi
		mNEgIsJQ4/WsOuJmfXGvTnrZ5rr6ZO/BJYK8C1gSbVUYFwxcX0XzOoZO/iWdAO4B
		Pv5vDL/k2U2nlSrzEdwQqBtc47WnyvsQKjTBpQmzKgojoAr7GLsAezvgmROu7Ecv
		LKrhYWe4rYez5GIObyyLLg5U3wE6ckzXosG4N9cT86unL1EhHqibTf0oFaTcrLRX
		8bA9Jgf8ubmaBB44UyDY
		-----END CERTIFICATE-----
		[root@lcsqaappintranet cgomez]#
		
Exportamos del p12 el Certificado::

	# openssl pkcs12 -in keystore.p12  -nokeys -out cert.crt
		Enter Import Password:r00tme
		MAC verified OK
	
	
Consultamos el certificado exportado::

	# openssl pkcs12 -in keystore.p12  -nokeys -out cert.crt
	
		Enter Import Password:r00tme
		MAC verified OK
		[root@lcsqaappintranet cgomez]# ls
		cert.crt  keystore.p12  lcsqaappintranet.credicard.com.ve.jks  lcsqaappintranet.credicard.com.ve.jks.orig  sample.war
		[root@lcsqaappintranet cgomez]# openssl x509 -noout -text -in cert.crt
		Certificate:
			Data:
				Version: 3 (0x2)
				Serial Number: 2101122747862202778 (0x1d28b001c7d1b99a)
			Signature Algorithm: sha256WithRSAEncryption
				Issuer: C=VE, ST=Distrito Capital, L=Caracas, O=Consorcio Credicard CA, OU=Criptografia, CN=Consorcio Credicard CA
				Validity
					Not Before: Jan 11 19:34:00 2022 GMT
					Not After : Jan 11 19:34:00 2023 GMT
				Subject: C=VE, ST=Distrito Capital, L=Caracas, O=Consorcio Credicard C.A, OU=Criptografia, CN=lcsqaappintranet.credicard.com.ve
				Subject Public Key Info:
					Public Key Algorithm: rsaEncryption
						Public-Key: (2048 bit)
						Modulus:
							00:bd:5a:6e:3b:26:db:09:0c:82:1b:68:16:e1:1c:
							9b:16:b1:12:1d:a8:1d:4d:17:d3:65:5e:7a:28:09:
							67:1b:cb:39:4f:95:21:1e:a0:b3:ae:d5:4a:5b:c8:
							09:54:93:d3:6c:18:47:94:c3:ce:28:ac:6f:ae:1d:
							ac:cd:78:4d:d6:41:85:3f:8f:f0:54:68:73:e9:48:
							19:93:44:15:10:cd:33:88:0e:17:94:df:d8:0a:c1:
							ec:f2:52:8e:e4:c4:64:a8:23:22:f8:13:96:e8:69:
							48:7a:26:89:0f:62:bd:3f:85:88:00:31:e9:64:be:
							b7:61:19:cb:57:32:91:5a:8d:4e:ef:63:97:91:6b:
							a4:f2:c9:d0:3a:04:00:26:55:99:e4:7e:14:21:d3:
							05:c9:9b:6e:b2:7f:3e:c3:8b:d4:41:78:8d:0d:44:
							72:a5:53:0b:04:67:ef:7a:6e:af:36:c1:f3:71:66:
							fd:e0:8f:de:ee:8f:d9:d4:0e:8f:31:fe:a3:25:80:
							f1:3b:b4:3f:46:cb:df:87:56:74:99:2c:61:ab:a9:
							b7:29:cb:e2:70:a9:f8:dc:56:37:ec:27:52:ba:93:
							b9:68:f7:c5:c8:31:14:05:bc:8b:b7:b4:3e:8e:6f:
							dc:1e:71:ca:4a:c0:eb:44:66:47:c5:20:16:a9:44:
							da:3b
						Exponent: 65537 (0x10001)
				X509v3 extensions:
					X509v3 Basic Constraints:
						CA:FALSE
					X509v3 Key Usage:
						Digital Signature, Non Repudiation, Key Encipherment
					X509v3 Subject Alternative Name:
						DNS:lcsqaappintranet.credicard.com.ve, DNS:intragestionqa.credicard.com.ve, DNS:intragestionqa, DNS:lcsqaappintranet, IP Address:10.134.0.81
			Signature Algorithm: sha256WithRSAEncryption
				 19:ac:04:14:3a:d5:d6:71:4d:95:e7:e0:ba:d2:56:32:08:e9:
				 81:34:ab:0e:2c:3d:cf:86:33:a2:d2:d3:d0:7e:f1:13:0e:79:
				 fc:b8:dc:e6:40:fb:b2:9c:9b:35:51:ab:9e:d2:23:d0:7e:63:
				 0b:b8:93:f3:e4:4e:dd:dd:20:82:15:7f:8f:04:56:ef:47:e3:
				 bc:ff:d0:ca:3e:a3:60:fa:05:b1:fa:93:90:41:ba:30:6a:e3:
				 41:b9:fc:9a:e1:30:cb:d9:b6:cd:19:48:47:2e:ec:c4:60:b6:
				 ff:25:f9:0e:c6:5a:d5:48:eb:93:6f:61:ea:89:59:f7:24:72:
				 b1:06:78:a4:ab:c5:71:4b:4c:a4:ff:af:e7:be:dd:c1:dd:d6:
				 2a:8f:69:1f:ee:83:47:6a:78:e6:c3:5a:d2:31:a3:fd:57:78:
				 44:0f:a2:f5:1d:9b:e2:53:d7:ed:2a:0d:2a:97:70:49:f4:fa:
				 57:c0:ed:b5:0f:82:d9:0f:cf:df:a5:86:e6:65:8a:c1:f0:ce:
				 a8:cf:0e:f0:fc:cb:e5:f8:51:93:01:09:32:f5:b5:0b:dc:d6:
				 67:b7:9c:4f:64:43:4a:7b:3b:24:78:c6:00:9d:ac:c6:d7:37:
				 3b:6e:e9:3e:e8:89:bc:14:80:ec:36:ee:c3:cc:b6:24:37:c8:
				 17:a7:32:54




Exportamos del p12 el KEY::

	# openssl pkcs12 -in keystore.p12  -nodes -nocerts -out cert.key
		Enter Import Password:r00tme
		MAC verified OK

Consultamos el KEY del certificado exportado::

	# openssl rsa -check -in cert.key
	
		RSA key ok
		writing RSA key
		-----BEGIN RSA PRIVATE KEY-----
		MIIEowIBAAKCAQEAvVpuOybbCQyCG2gW4RybFrESHagdTRfTZV56KAlnG8s5T5Uh
		HqCzrtVKW8gJVJPTbBhHlMPOKKxvrh2szXhN1kGFP4/wVGhz6UgZk0QVEM0ziA4X
		lN/YCsHs8lKO5MRkqCMi+BOW6GlIeiaJD2K9P4WIADHpZL63YRnLVzKRWo1O72OX
		kWuk8snQOgQAJlWZ5H4UIdMFyZtusn8+w4vUQXiNDURypVMLBGfvem6vNsHzcWb9
		4I/e7o/Z1A6PMf6jJYDxO7Q/Rsvfh1Z0mSxhq6m3KcvicKn43FY37CdSupO5aPfF
		yDEUBbyLt7Q+jm/cHnHKSsDrRGZHxSAWqUTaOwIDAQABAoIBACs+VsRNiGJVp/UI
		XYlFlimlgMSjGyX7Ff0liXJRS2nujIUfQrQS8VYxQc0aLv9Qz0z1couH+DITx2GV
		R1yZZ/VRe1Pb1IACZs5U9/pI5yKyKl2dEkeeo2E5jpp8vkOCkpZPh/Htz13+hV3Y
		JR8NZrj2Duw0ed/XKlwTnvuoAcgS7uL/JB96Lf3XFaTBJ1sPQF4fCL7ym0RquFvp
		ochFV/S+B0b5ycvqfQ2/+hkFG7v1KMovylmojCuCppB1JQ9l3dSj5WzYBjGct4VP
		er96jUHlPYEfxRYgX6Fhe6ghujKtP7ORrYpM5xnODaytCvxlEx7VZ+akXZsPUcGW
		n5vIpoECgYEA4ugNvgcmykfDe2OK/EnPUThKaWDxCDKZB0wR0M+z/RoINX5Ldx2g
		QgWcXFnyNmcpD0N1jreRSZ2yTGyZEd0Eu7rABJBWKsuWxjdTx3iOkJjQUyLiwn0w
		Wpnpadf4ZBbee7eq/QIx2kZoR12Z95zJinbzoS1qUe/SDbY8EFY57cECgYEA1aG8
		QErKLJSIAgWDU7weXjNaZLYvwpjFdxLsqA62OgPym6aO7DWtq+WVPKosGmCBs8S1
		IeRxM7whKhPgDr5OQinqeD6yO4T+GzXtKHeakkavDP59moWxG9OJMo5aTZQZ6RBK
		bAjIG9HieyRDXg+8L8FPsVl48Ky8X6wOPBfkPvsCgYBnAJwsZSawsH8GphtTh1X7
		MqhkycLgy8c3zspPldnIzWZokhpDykkTb2SZb6NKGu5CpYbZ8G6dkl573thliYU6
		iv3blIHpD140QK1hYVKmRRhchPuW+ilXF4Mjrwxsswzv8GJIVBS5VzjDHLRl+OBs
		YK8bvXgEFe+ulckSSXImgQKBgCMewan4IaCOkoVyjpJ3fK6T1qpz4Qomv1/B9rHy
		KTcEax/3k8t1T6XQymX8u99iOjBpiDWYLpwIs5MNTWpfEtKBvZAjDn4GcRfcF67t
		arXddO238MI0dFdUwVtUV7glPtU33mRAVVVtfcQsw/50q8VWDFnlkaJPY3B/AqAS
		dW19AoGBAMR3E0NDE2O1TEqmQhEvTlRydFl7zSUJY1xi7vfBMLzD5MYg7UrI1mOq
		tvFHeOVT2h6/6X6qyTQf6LRLj9gqPZgWJm3mlUq6k5gzxvMF9z2QiAL4Hef25HZF
		0FSSRvx3mdiVzB3er/+4KS7qWHEAjIIQ9ODxsKLwLtFEJwOOOTK5
		-----END RSA PRIVATE KEY-----
		[root@lcsqaappintranet cgomez]#
