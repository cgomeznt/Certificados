Troubleshooting con OpenSSL
============================

Pasos para realizar un Troubleshooting para garantizar que la Private Key realmente es una private key, que se pueda hacer la consulta del certificado y del certificado publico de la CA, verificar el certificado contra el certificado publico de la CA y por ultimo simular un cliente la petición del certificado.

Guía rápida
+++++++++++++++++++

::

	cat srvutils.key 

	openssl x509 -in srvutils.crt -noout -text

	openssl x509 -in ca-fabric.crt -noout -text

	openssl verify -CAfile ca-fabric.crt srvutils.crt

	openssl s_client -connect localhost:8080 -CAfile ca-fabric.crt


Comandos utilizados y sus salidas
++++++++++++++++++++++++++++++++++++

Aquí debemos garantizar que la ver BEGIN PRIVATE KEY y END PRIVATE KEY::

	# cat srvutils.key 
	-----BEGIN PRIVATE KEY-----
	MIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQCbCL5QHmB93SS0
	kZpdq3HACWkiHDozEHhlk6NG+mB0p54rSyIPGlHVL733472dIwwg77jQv9BSMr67
	+2G0lQCAAW2GgBGwnwLSvSp7R95lCEHWU6zWKrxycbArgeeUx9mUQEJcAHKidMSe
	uWasWMbAAERXo/lILJhKy0e3WVDMck+f1QYbjoxXOfVHQWeVljXKeKHwS/ze4FjD
	i8P7ZbVlMiWqmV6Cnkrn4IGtBS2MbokJk8Kx2o2ZErGexiGMGn9s/euIiAmT3eI7
	EpY+KbURlzNBnhEjpMXVkzKhm4pWnrwaqYKZG91LftpImHMXU86imWeNlyq7wBUy
	nzbWOttFAgMBAAECggEAP/c7jpL5/PRhUJ9CsaMrK7C2T/yVhrwk8MQZeg+T/openssl verify -CAfile ca-fabric.crt srvutils.crtI2s
	FPDK/LA/U1Z/aufsNAlh17UQ7BA4Q7HsQGNXzMotiqMMLZJpuhXhdFHYVUUx3T2q
	7GNZzIOVfSKrLUhL5HcQrrpUpuEIaz8CYPreAf4fAtkZWY+uKrk7nKCC1oNjcvaS
	vCvErqotOso8x2Qq4IiAAtLBa+3PRmgrm2YaJMs+kAkuzqNzXufEEzFjI6Yggh+O
	1VcVkKGrOriAZWWeUCTlH2V4C+J3x4qI7YDSa9/UalgRit0KPR98gI5ANJ/CM9zb
	ADxFTLlcsbkSHNx/sAEHHwKAulivYcQzkIXKk3yvgQKBgQDJTh+rzTBPe4l2cqPf
	dpkbivl1Ordb5bFIO1CVRgY56Na6UqkMdY9h6dY2AP6wGMaM8vzaoetooivFbt1V
	pG5opk0rxx6xiSXICZkH6HTuPlIFKPpFtAu1WxIqnZadQTk1eRjeJ8oZ9mkmh998
	tXIdr0ZznIMmchdsn0m268BR5wKBgQDFKDed6yj0tP3UZqHnNJ5FdsKxuHdJh6fG
	RgS/uPUctFTD4oVba6Ca4HM19ncmuj6ykwyndYIeVWPJq3WjGjxmHFPSX+koYfMF
	+G0iqg8OOihwHQvQCRsYDVZOk+zGuZvq8B0pqMYCi7bW/cgkndxSbVNCIoXWSmHe
	AMPSr/Bb8wKBgQCOM3Ct9OFWlEbTdEIMfgPD8BUt2Y9jDEuCYdwXxoJpb/AXaILg
	OT9TBFL8jRFcpfPD53X0n4LixAQ1kI4rjF8t76P988fb06zrtNP0QSIwlbdsy7iX
	eor2zmFz1yRo64UVH/kQHX1nf6hhvoOB3c7B52nWC06d3uRrJ85zt++AKwKBgFhj
	SFPW6lySi71eabUipNYVgQF15pyjYXcFSvm87L56pgnPkuPCY5UrjNsjbJWDJ7qd
	LC4jAzugIoV2Bd4iU/OfPYDyGLBPAUmq7xp8TRWewyRIEVSp0Gi/CfNeY+dPrPPt
	w9U6YdMgWc8WpVStJOobMxlSKthALpH9m8znrYU7AoGAIn+LKslYzWNW7JBE2Nrk
	v4wfaSqgRd+a4K9F+d0kphH8UVLnjLxBgTIwFcWzyeUIiPxdwfqJoLaTrI71S/iO
	hmFiW3mIHp9PPbbh+9t3XDZJImObIOX/jZXET0PPL5yAmZqHu7O3bKXaPVE4wopenssl verify -CAfile ca-fabric.crt srvutils.crtFhx
	wrsMJ6oZigv1sU9MhWbqRrM=
	-----END PRIVATE KEY-----



Consultar el certificado::

	# openssl x509 -in srvutils.crt -noout -text
	Certificate:
	    Data:
		Version: 3 (0x2)
		Serial Number:
		    97:da:35:a0:a1:32:fb:2d
	    Signature Algorithm: sha256WithRSAEncryption
		Issuer: C=VE, ST=DC, L=DC, O=Default Fabric ltd, OU=Support Criptography, CN=criptography/emailAddress=root@fabric.com
		Validity
		    Not Before: Sep  1 21:47:21 2021 GMT
		    Not After : Mar  5 21:47:21 2022 GMT
		Subject: C=VE, ST=DC, L=Caracas, O=PERSONAL, OU=TI, CN=srvutils
		Subject Public Key Info:
		    Public Key Algorithm: rsaEncryption
		        Public-Key: (2048 bit)
		        Modulus:
		            00:9b:08:be:50:1e:60:7d:dd:24:b4:91:9a:5d:ab:
		            71:c0:09:69:22:1c:3a:33:10:78:65:93:a3:46:fa:
		            60:74:a7:9e:2b:4b:22:0f:1a:51:d5:2f:bd:f7:e3:
		            bd:9d:23:0c:20:ef:b8:d0:bf:d0:52:32:be:bb:fb:
		            61:b4:95:00:80:01:6d:86:80:11:b0:9f:02:d2:bd:
		            2a:7b:47:de:65:08:41:d6:53:ac:d6:2a:bc:72:71:
		            b0:2b:81:e7:94:c7:d9:94:40:42:5c:00:72:a2:74:
		            c4:9e:b9:66:ac:58:c6:c0:00:44:57:a3:f9:48:2c:
		            98:4a:cb:47:b7:59:50:cc:72:4f:9f:d5:06:1b:8e:
		            8c:57:39:f5:47:41:67:95:96:35:ca:78:a1:f0:4b:
		            fc:de:e0:58:c3:8b:c3:fb:65:b5:65:32:25:aa:99:
		            5e:82:9e:4a:e7:e0:81:ad:05:2d:8c:6e:89:09:93:
		            c2:b1:da:8d:99:12:b1:9e:c6:21:8c:1a:7f:6c:fd:
		            eb:88:88:09:93:dd:e2:3b:12:96:3e:29:b5:11:97:
		            33:41:9e:11:23:a4:c5:d5:93:32:a1:9b:8a:56:9e:
		            bc:1a:a9:82:99:1b:dd:4b:7e:da:48:98:73:17:53:
		            ce:a2:99:67:8d:97:2a:bb:c0:15:32:9f:36:d6:3a:
		            db:45
		        Exponent: 65537 (0x10001)
		X509v3 extensions:
		    X509v3 Basic Constraints: 
		        CA:FALSE
		    X509v3 Key Usage: 
		        Digital Signature, Non Repudiation, Key Encipherment
		    X509v3 Subject Alternative Name: 
		        DNS:srvscmutils.fabric.local, DNS:monitoreo.fabric.local, IP Address:192.168.0.20
	    Signature Algorithm: sha256WithRSAEncryption
		 17:ce:e9:84:97:13:2a:4b:9c:6c:53:aa:8d:a4:80:53:7f:dc:
		 61:95:4e:a7:e7:1f:da:36:df:51:4c:64:dc:e8:b4:d3:fa:97:
		 01:24:78:69:11:9e:de:19:07:b7:35:b0:c8:8e:04:ef:ea:0b:
		 1d:77:f1:cd:64:2c:a2:0b:7b:17:5f:98:96:47:fa:dc:16:53:
		 5d:1d:f8:62:21:42:8e:15:09:73:30:dd:ff:b0:20:c7:c0:83:
		 77:ab:90:50:98:7f:33:48:80:c7:7c:65:69:f6:59:d1:2f:02:
		 24:1c:bd:16:9c:1e:ff:d5:c3:51:7a:06:e4:ca:35:d1:8d:b1:
		 70:96:1f:6d:2b:ba:99:7b:93:6c:98:5c:ce:c1:db:54:f9:2a:
		 34:cf:af:4d:1e:40:2b:b4:4e:1c:e4:35:0c:63:b4:e1:b9:3a:
		 b5:03:7a:3a:1a:93:1e:21:90:18:3a:30:08:08:41:5f:9a:6d:
		 97:75:be:a4:b6:48:fd:1a:60:b4:9e:2c:52:b0:db:65:0f:61:
		 17:e6:3f:cc:1b:71:dc:79:ee:98:e6:bb:a7:26:20:e7:b9:d8:
		 ad:6a:4b:96:7f:a6:95:20:4e:d8:31:96:f5:46:71:4e:6a:50:
		 12:04:12:84:a2:20:21:9e:89:60:f1:5d:cf:d3:82:e0:be:3c:
		 3c:3c:ab:a0


Consultar el certificado publico de la CA y que veamos que realmente dice CA:TRUE::

	# openssl x509 -in ca-fabric.crt -noout -text
	Certificate:
	    Data:
		Version: 3 (0x2)
		Serial Number:
		    b3:b1:bb:5c:e2:cd:c2:d8
	    Signature Algorithm: sha256WithRSAEncryption
		Issuer: C=VE, ST=DC, L=DC, O=Default Fabric ltd, OU=Support Criptography, CN=criptography/emailAddress=root@fabric.com
		Validity
		    Not Before: Sep  1 21:42:35 2021 GMT
		    Not After : Aug 30 21:42:35 2031 GMT
		Subject: C=VE, ST=DC, L=DC, O=Default Fabric ltd, OU=Support Criptography, CN=criptography/emailAddress=root@fabric.com
		Subject Public Key Info:
		    Public Key Algorithm: rsaEncryption
		        Public-Key: (2048 bit)
		        Modulus:
		            00:df:a5:6a:6f:a1:29:43:3f:42:11:57:99:bb:53:
		            a3:d2:fe:9b:8a:d5:8f:db:e0:e8:44:b8:f9:83:20:
		            00:c5:85:22:f4:84:00:50:db:b3:0b:31:45:aa:15:
		            6c:09:b8:77:2d:cb:4b:17:0d:d4:fd:a8:ed:bd:a3:
		            cf:3e:35:07:a3:da:fa:5c:cd:ef:3b:e4:8e:f2:9c:
		            db:f6:94:e9:f7:66:fb:df:cb:cf:a1:82:a7:a4:00:
		            d0:1e:0a:41:9c:58:d2:99:b4:13:b1:83:30:17:58:
		            0f:4f:53:b9:5c:58:c3:37:03:bc:b7:42:4f:cb:c1:
		            b0:56:9b:73:95:bb:5d:1a:f0:91:0e:21:5d:53:61:
		            01:f0:c8:d3:00:1c:f4:4d:16:f0:e9:6c:11:84:26:
		            f4:ab:7b:65:bf:24:8d:62:27:0c:14:c1:3e:71:56:
		            4a:2e:b7:a0:1f:d0:64:ed:58:1a:d8:5a:7f:8a:da:
		            1c:c1:2b:71:31:66:a5:5d:01:f7:d2:f8:0c:d3:c3:
		            00:87:2f:67:f8:1e:38:b2:0e:d9:c6:99:a9:64:fb:
		            fc:21:94:99:6e:32:32:a5:0d:97:a0:9e:25:9c:4b:
		            f3:a6:27:8d:99:e2:e5:c0:71:8d:8c:9d:66:9c:e0:
		            8c:46:85:52:02:f8:39:9f:d3:25:79:4a:f8:05:d9:
		            e4:85
		        Exponent: 65537 (0x10001)
		X509v3 extensions:
		    X509v3 Subject Key Identifier: 
		        73:F6:21:33:D6:25:F6:D4:33:18:0F:BB:CE:C7:31:A5:50:84:63:9A
		    X509v3 Authority Key Identifier: 
		        keyid:73:F6:21:33:D6:25:F6:D4:33:18:0F:BB:CE:C7:31:A5:50:84:63:9A

		    X509v3 Basic Constraints: 
		        CA:TRUE
	    Signature Algorithm: sha256WithRSAEncryption
		 ad:eb:6d:5b:4a:44:55:90:18:20:57:8a:85:8a:b8:9d:fc:b1:
		 38:ae:0b:40:9b:c0:74:6f:99:d7:29:c2:19:a4:8e:ff:a8:ef:
		 2d:25:02:f4:df:e6:27:99:70:3f:00:0f:eb:91:49:51:60:35:
		 23:fc:3a:4d:14:dd:e4:67:a9:a3:fe:17:3e:42:dc:66:22:dc:
		 d9:91:53:9e:35:ae:06:27:c6:73:36:81:1a:30:97:c5:f2:6c:
		 b6:98:11:7c:74:dc:c4:ca:74:c4:10:aa:9a:85:f3:cd:1e:71:
		 7f:af:90:10:3e:2d:d2:c1:7e:4f:34:8e:ea:b0:3e:73:78:05:
		 81:62:95:bf:88:fb:c9:14:ab:8f:bd:df:24:33:fd:44:c2:30:
		 a0:30:72:af:9d:b0:b7:7c:d2:cc:0f:4b:f6:e4:9e:31:7e:b0:
		 40:de:ef:dd:ff:b3:4e:49:b7:d6:65:e5:7e:ce:bc:8f:4d:ce:
		 b6:b5:22:5c:a3:d7:25:90:92:16:69:b4:ad:7a:1b:ab:44:7b:
		 62:24:c3:7c:6b:18:9e:cd:f8:70:a2:4a:83:1d:6d:2e:36:5e:
		 44:8e:f7:69:f6:6e:62:b9:57:c6:29:94:f7:73:b7:2d:3d:a7:
		 15:6f:89:54:11:55:38:28:47:86:4e:2f:cc:be:1e:8d:9d:a5:
		 dc:b2:2e:53



Verificar el certificado contra el certificado publico de la CA::

	# openssl verify -CAfile ca-fabric.crt srvutils.crt 
	srvutils.crt: OK


Simular un cliente la petición del certificado::

	# openssl s_client -connect 192.168.1.20:443 -CAfile ca-fabric.crt

	CONNECTED(00000003)
	depth=1 C = VE, ST = DC, L = DC, O = Default Fabric ltd, OU = Support Criptography, CN = criptography, emailAddress = root@fabric.com
	verify return:1
	depth=0 C = VE, ST = DC, L = Caracas, O = PERSONAL, OU = TI, CN = srvutils
	verify return:1
	---
	Certificate chain
	 0 s:/C=VE/ST=DC/L=Caracas/O=PERSONAL/OU=TI/CN=srvutils
	   i:/C=VE/ST=DC/L=DC/O=Default Fabric ltd/OU=Support Criptography/CN=criptography/emailAddress=root@fabric.com
	---
	Server certificate
	-----BEGIN CERTIFICATE-----
	MIID2jCCAsKgAwIBAgIJAJfaNaChMvstMA0GCSqGSIb3DQEBCwUAMIGaMQswCQYD
	VQQGEwJWRTELMAkGA1UECAwCREMxCzAJBgNVBAcMAkRDMRswGQYDVQQKDBJEZWZh
	dWx0IEZhYnJpYyBsdGQxHTAbBgNVBAsMFFN1cHBvcnQgQ3JpcHRvZ3JhcGh5MRUw
	EwYDVQQDDAxjcmlwdG9ncmFwaHkxHjAcBgkqhkiG9w0BCQEWD3Jvb3RAZmFicmlj
	LmNvbTAeFw0yMTA5MDEyMTQ3MjFaFw0yMjAzMDUyMTQ3MjFaMF8xCzAJBgNVBAYT
	AlZFMQswCQYDVQQIDAJEQzEQMA4GA1UEBwwHQ2FyYWNhczERMA8GA1UECgwIUEVS
	U09OQUwxCzAJBgNVBAsMAlRJMREwDwYDVQQDDAhzcnZ1dGlsczCCASIwDQYJKoZI
	hvcNAQEBBQADggEPADCCAQoCggEBAJsIvlAeYH3dJLSRml2rccAJaSIcOjMQeGWT
	o0b6YHSnnitLIg8aUdUvvffjvZ0jDCDvuNC/0FIyvrv7YbSVAIABbYaAEbCfAtK9
	KntH3mUIQdZTrNYqvHJxsCuB55TH2ZRAQlwAcqJ0xJ65ZqxYxsAARFej+UgsmErL
	R7dZUMxyT5/VBhuOjFc59UdBZ5WWNcp4ofBL/N7gWMOLw/tltWUyJaqZXoKeSufg
	ga0FLYxuiQmTwrHajZkSsZ7GIYwaf2z964iICZPd4jsSlj4ptRGXM0GeESOkxdWT
	MqGbilaevBqpgpkb3Ut+2kiYcxdTzqKZZ42XKrvAFTKfNtY620UCAwEAAaNdMFsw
	CQYDVR0TBAIwADALBgNVHQ8EBAMCBeAwQQYDVR0RBDowOIIYc3J2c2NtdXRpbHMu
	ZmFicmljLmxvY2FsghZtb25pdG9yZW8uZmFicmljLmxvY2FshwTAqAAUMA0GCSqG
	SIb3DQEBCwUAA4IBAQAXzumElxMqS5xsU6qNpIBTf9xhlU6n5x/aNt9RTGTc6LTT
	+pcBJHhpEZ7eGQe3NbDIjgTv6gsdd/HNZCyiC3sXX5iWR/rcFlNdHfhiIUKOFQlz
	MN3/sCDHwIN3q5BQmH8zSIDHfGVp9lnRLwIkHL0WnB7/1cNRegbkyjXRjbFwlh9t
	K7qZe5NsmFzOwdtU+So0z69NHkArtE4c5DUMY7ThuTq1A3o6GpMeIZAYOjAICEFf
	mm2Xdb6ktkj9GmC0nixSsNtlD2EX5j/MG3Hcee6Y5runJiDnuditakuWf6aVIE7Y
	MZb1RnFOalASBBKEoiAhnolg8V3P04Lgvjw8PKug
	-----END CERTIFICATE-----
	subject=/C=VE/ST=DC/L=Caracas/O=PERSONAL/OU=TI/CN=srvutils
	issuer=/C=VE/ST=DC/L=DC/O=Default Fabric ltd/OU=Support Criptography/CN=criptography/emailAddress=root@fabric.com
	---
	No client certificate CA names sent
	Peer signing digest: SHA512
	Server Temp Key: ECDH, P-256, 256 bits
	---
	SSL handshake has read 1681 bytes and written 415 bytes
	---
	New, TLSv1/SSLv3, Cipher is ECDHE-RSA-AES256-GCM-SHA384
	Server public key is 2048 bit
	Secure Renegotiation IS supported
	Compression: NONE
	Expansion: NONE
	No ALPN negotiated
	SSL-Session:
	    Protocol  : TLSv1.2
	    Cipher    : ECDHE-RSA-AES256-GCM-SHA384
	    Session-ID: D604DE5A33E8065704B32FEA472223850DC1E1515809EAD313109FBC9B54AF97
	    Session-ID-ctx: 
	    Master-Key: 394C87E565B37F86E5D5D519A077EDBD8C5B3C39DB4AD4AAA9AC93EEF4349C388630072D2CAFB771180A4CFF0E5E91D5
	    Key-Arg   : None
	    Krb5 Principal: None
	    PSK identity: None
	    PSK identity hint: None
	    TLS session ticket lifetime hint: 300 (seconds)
	    TLS session ticket:
	    0000 - 1c b3 d4 87 0d 80 1d 49-be 65 3c d9 6e 3f 43 62   .......I.e<.n?Cb
	    0010 - ca 8c 17 8e 6b d2 21 ac-d5 a0 a7 0b db 3d 20 70   ....k.!......= p
	    0020 - ae 3c 76 88 1a b2 ef f5-3f 8d cd c1 0f 66 c5 11   .<v.....?....f..
	    0030 - 40 7d 18 b7 7d 39 9d 2b-ef 92 40 a5 53 e1 78 a6   @}..}9.+..@.S.x.
	    0040 - 8b 26 4d fc 1c fd de 4a-8e 69 63 f4 42 bf cc f7   .&M....J.ic.B...
	    0050 - 94 fd 1d ff f4 81 06 bd-c8 34 67 ca 2f 2c a4 e3   .........4g./,..
	    0060 - 6a e4 8c 9b 7a c8 e2 4a-27 de 88 b2 c0 6f dc cf   j...z..J'....o..
	    0070 - 9a 5b 4b 40 58 05 0d e6-03 c3 46 2f 49 c3 26 e7   .[K@X.....F/I.&.
	    0080 - 8a 4e d7 28 f4 11 72 6a-9f d6 29 88 f5 bc cf de   .N.(..rj..).....
	    0090 - ce f7 0a 97 19 50 59 fc-6a 48 c7 44 75 60 0c ce   .....PY.jH.Du`..
	    00a0 - 20 58 4e 00 31 23 95 52-d2 cf 43 55 9f 74 31 3d    XN.1#.R..CU.t1=
	    00b0 - ea e2 9e 6a ec 2c e4 70-dd af a1 d2 3d 80 43 60   ...j.,.p....=.C`

	    Start Time: 1630619652
	    Timeout   : 300 (sec)
	    Verify return code: 0 (ok)
	---

