

Hacemos una consulta con openssl hacia www.credicard.com.ve::

	openssl s_client -connect www.credicard.com.ve:443

y vemos que el resultado del SSL es exitoso::

	---
	SSL handshake has read 4551 bytes and written 438 bytes
	Verification: OK
	---


Ahora vamos a simular que no tenemos el certificados publico de Digicert en el equipo en donde estamos ejecutando el openssl. ¿y por que el certificado publico de Digicert? porque el certificado de www.credicard.com.ve esta firmado por Digicert.

Esto fue realizado en un Debian 10, vamos a mover todos los certificados publicos de Digicert del deposito de certificados para otra ruta::

	mv /etc/ssl/certs/DigiCert_* .

Hacemos nuevamente la consulta con openssl hacia www.credicard.com.ve::

	openssl s_client -connect www.credicard.com.ve:443

y vemos que ahora tenemos error de handshake del resultado del SSL::

	---
	SSL handshake has read 4551 bytes and written 438 bytes
	Verification error: self signed certificate in certificate chain
	---

Consultamos en donde movimos los certificados de Digicert::

	ls
	ca-certificates.conf		 DigiCert_Assured_ID_Root_G3.pem  DigiCert_Global_Root_G3.pem
	DigiCert_Assured_ID_Root_CA.pem  DigiCert_Global_Root_CA.pem	  DigiCert_High_Assurance_EV_Root_CA.pem
	DigiCert_Assured_ID_Root_G2.pem  DigiCert_Global_Root_G2.pem	  DigiCert_Trusted_Root_G4.pem


Y ahora vamos ejecutar el mismo comando de openssl, pero ahora le vamos a decir en donde esta el certificado publico de la CA de Digicert::

	openssl s_client -connect www.credicard.com.ve:443 -CAfile /root/DigiCert_Global_Root_CA.pem

y vemos que el resultado del SSL es exitoso::

	---
	SSL handshake has read 4551 bytes and written 438 bytes
	Verification: OK
	---

