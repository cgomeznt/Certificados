Certificados Digitales Letsencrypt
================================================

¿Qué es Let's Encrypt?
++++++++++++++++++++++++

Let’s Encrypt es una AC. Para obtener un certificado para tu dominio de sitio web de Let’s Encrypt, tienes que demonstrar control sobre ese dominio. Con Let’s Encrypt, puedes hacer esto con software que usa el protocolo ACME, el cual típicamente corre en tu hospedaje de web.

Let's Encrypt es una nueva autoridad de certificación: es gratuita, automatizada y abierta. Podría ser una opción para proteger los servidores Zimbra con un certificado SSL válido; sin embargo, tenga en cuenta que es una versión Beta por ahora. Algunas cosas no pueden funcionar o tienen problemas, así que úselo bajo su propio riesgo.

Los certificados Letsencrypt son certificados comerciales gratuitos que se renuevan cada 2 meses y ya son reconocidos por los navegadores y clientes de correo actuales.

Para instalar un certificado letsencrypt se requiere que el servidor zimbra esté publicado a internet con una IP pública y resolución de DNS; ya sea directa o a través de NAT

¿Qué es Certbot?
+++++++++++++++

Certbot es una herramienta de software gratuita y de código abierto para usar automáticamente certificados Let's Encrypt en sitios web administrados manualmente para habilitar HTTPS.

Tenemos dos (2) formas de hacer la instalacion de **certbot**, la primera con la ayuda de los repositorios EPEL de CentOS y la otra utilizando snap desde el sitio oficial 

Instalando **certbot** con yum
++++++++++++++++++++++++++++++
::

	yum -y install epel-release
	yum -y install python-certbot-apache


Instalando **certbot** con snap
++++++++++++++++++++++++++++++
::

	yum install snapd

	systemctl enable --now snapd.socket

	ln -s /var/lib/snapd/snap /snap

	snap install core; sudo snap refresh core

	snap install --classic certbot

	ln -s /snap/bin/certbot /usr/bin/certbot


Creando el certificado firmado por letsencrypt
++++++++++++++++++++++++++++++++++++++++++++++
::

	# certbot certonly -d mail.e-deus.cf -m cgomeznt@gmail.com --standalone
	Saving debug log to /var/log/letsencrypt/letsencrypt.log
	Requesting a certificate for mail.e-deus.cf and e-deus.cf

	Successfully received certificate.
	Certificate is saved at: /etc/letsencrypt/live/mail.e-deus.cf-0003/fullchain.pem
	Key is saved at:         /etc/letsencrypt/live/mail.e-deus.cf-0003/privkey.pem
	This certificate expires on 2021-10-05.
	These files will be updated when the certificate renews.
	Certbot has set up a scheduled task to automatically renew this certificate in the background.

Si necesita tener varios nombres de host en el mismo SSL, entonces un Multi-SAN, SSL, por favor ejecute en su lugar, donde -d son sus dominios:::

	certbot  certonly  -d mail.e-deus.cf -d e-deus.cf -m cgomeznt@e-deus.cf--standalone

¿Dónde están los archivos de certificado SSL?
++++++++++++++++++++++++++++++

Puede encontrar todos sus archivos en /etc/letsencrypt/live/$domain, donde $domain es el fqdn que utilizó durante el proceso::

	ls -l /etc/letsencrypt/live/mail.e-deus.cf-0003/
	total 4
	lrwxrwxrwx 1 zimbra root  43 jul  7 16:56 cert.pem -> ../../archive/mail.e-deus.cf-0003/cert1.pem
	lrwxrwxrwx 1 zimbra root  44 jul  7 16:56 chain.pem -> ../../archive/mail.e-deus.cf-0003/chain1.pem
	lrwxrwxrwx 1 zimbra root  48 jul  7 16:56 fullchain.pem -> ../../archive/mail.e-deus.cf-0003/fullchain1.pem
	lrwxrwxrwx 1 zimbra root  46 jul  7 16:56 privkey.pem -> ../../archive/mail.e-deus.cf-0003/privkey1.pem
	-rw-r--r-- 1 zimbra root 692 jul  7 16:56 README

cert.pem es el certificado

chain.pem es la cadena

fullchain.pem es la concatenación de cert.pem + chain.pem

privkey.pem es la clave privada

Tenga en cuenta que la clave privada es solo para usted.

Cree la CA intermedia más la CA raíz adecuada
++++++++++++++++++++++++++++++++++++++++

Let's Encrypt es casi perfecto, pero durante los archivos que construyó el proceso, simplemente agregan el archivo chain.pem sin la CA raíz. Debe utilizar el certificado raíz IdenTrust y fusionarlo después de chain.pem

https://letsencrypt.org/certs/trustid-x3-root.pem.txt

Su chain.pem debería verse así::

	echo "-----BEGIN CERTIFICATE-----
	MIIDSjCCAjKgAwIBAgIQRK+wgNajJ7qJMDmGLvhAazANBgkqhkiG9w0BAQUFADA/
	MSQwIgYDVQQKExtEaWdpdGFsIFNpZ25hdHVyZSBUcnVzdCBDby4xFzAVBgNVBAMT
	DkRTVCBSb290IENBIFgzMB4XDTAwMDkzMDIxMTIxOVoXDTIxMDkzMDE0MDExNVow
	PzEkMCIGA1UEChMbRGlnaXRhbCBTaWduYXR1cmUgVHJ1c3QgQ28uMRcwFQYDVQQD
	Ew5EU1QgUm9vdCBDQSBYMzCCASIwDQYJKoZIhvcNAQEBBQADggEPADCCAQoCggEB
	AN+v6ZdQCINXtMxiZfaQguzH0yxrMMpb7NnDfcdAwRgUi+DoM3ZJKuM/IUmTrE4O
	rz5Iy2Xu/NMhD2XSKtkyj4zl93ewEnu1lcCJo6m67XMuegwGMoOifooUMM0RoOEq
	OLl5CjH9UL2AZd+3UWODyOKIYepLYYHsUmu5ouJLGiifSKOeDNoJjj4XLh7dIN9b
	xiqKqy69cK3FCxolkHRyxXtqqzTWMIn/5WgTe1QLyNau7Fqckh49ZLOMxt+/yUFw
	7BZy1SbsOFU5Q9D8/RhcQPGX69Wam40dutolucbY38EVAjqr2m7xPi71XAicPNaD
	aeQQmxkqtilX4+U9m5/wAl0CAwEAAaNCMEAwDwYDVR0TAQH/BAUwAwEB/zAOBgNV
	HQ8BAf8EBAMCAQYwHQYDVR0OBBYEFMSnsaR7LHH62+FLkHX/xBVghYkQMA0GCSqG
	SIb3DQEBBQUAA4IBAQCjGiybFwBcqR7uKGY3Or+Dxz9LwwmglSBd49lZRNI+DT69
	ikugdB/OEIKcdBodfpga3csTS7MgROSR6cz8faXbauX+5v3gTt23ADq1cEmv8uXr
	AvHRAosZy5Q6XkjEGB5YGV8eAlrwDPGxrancWYaLbumR9YbK+rlmM6pZW87ipxZz
	R8srzJmwN0jP41ZL9c8PDHIyh8bwRLtTcm1D9SZImlJnt1ir/md2cXjbDaJWFBM5
	JDGFoqgCWjBH4d1QB7wCCZAA62RjYJsWvIjJEubSfZGL+T0yjWW06XyxV3bqxbYo
	Ob8VZRzI9neWagqNdwvYkQsEjgfbKbYK7p2CNTUQ
	-----END CERTIFICATE-----">> /etc/letsencrypt/live/$HOSTNAME/chain.pem

Su chain.pem debería verse así::

	----- BEGIN CERTIFICATE ----- 
	YOURCHAIN 
	----- END CERTIFICATE ----- 
	----- BEGIN CERTIFICATE ----- 
	MIIDSjCCAjKgAwIBAgIQRK + wgNajJ7qJMDmGLvhAazANBgkqhkiG9w0BAQUFADA / 
	MSQwIgYDVQQKExtEaWdpdGFsIFNpZ25hdHVyZSBUcnVzdCBDby4xFzAVBgNVBAMT 
	DkRTVCBSb290IENBIFgzMB4XDTAwMDkzMDIxMTIxOVoXDTIxMDkzMDE0MDExNVow 
	PzEkMCIGA1UEChMbRGlnaXRhbCBTaWduYXR1cmUgVHJ1c3QgQ28uMRcwFQYDVQQD 
	Ew5EU1QgUm9vdCBDQSBYMzCCASIwDQYJKoZIhvcNAQEBBQADggEPADCCAQoCggEB 
	AN + v6ZdQCINXtMxiZfaQguzH0yxrMMpb7NnDfcdAwRgUi + DoM3ZJKuM / IUmTrE4O 
	rz5Iy2Xu / NMhD2XSKtkyj4zl93ewEnu1lcCJo6m67XMuegwGMoOifooUMM0RoOEq 
	OLl5CjH9UL2AZd + 3UWODyOKIYepLYYHsUmu5ouJLGiifSKOeDNoJjj4XLh7dIN9b 
	xiqKqy69cK3FCxolkHRyxXtqqzTWMIn / 5WgTe1QLyNau7Fqckh49ZLOMxt + / yUFw
	7BZy1SbsOFU5Q9D8 / RhcQPGX69Wam40dutolucbY38EVAjqr2m7xPi71XAicPNaD 
	aeQQmxkqtilX4 + U9m5 / wAl0CAwEAAaNCMEAwDwYDVR0TAQH / BAUwAwEB / zAOBgNV 
	HQ8BAf8EBAMCAQYwHQYDVR0OBBYEFMSnsaR7LHH62 + FLkHX / xBVghYkQMA0GCSqG 
	SIb3DQEBBQUAA4IBAQCjGiybFwBcqR7uKGY3Or + Dxz9LwwmglSBd49lZRNI + DT69 
	ikugdB / OEIKcdBodfpga3csTS7MgROSR6cz8faXbauX + 5v3gTt23ADq1cEmv8uXr 
	AvHRAosZy5Q6XkjEGB5YGV8eAlrwDPGxrancWYaLbumR9YbK + rlmM6pZW87ipxZz 
	R8srzJmwN0jP41ZL9c8PDHIyh8bwRLtTcm1D9SZImlJnt1ir / md2cXjbDaJWFBM5 
	JDGFoqgCWjBH4d1QB7wCCZAA62RjYJsWvIjJEubSfZGL + T0yjWW06XyxV3bqxbYo 
	Ob8VZRzI9neWagqNdwvYkQsEjgfbKbYK7p2CNTUQ 
	----- END CERTIFICATE -----

En resumen: chain.pem debe concatenarse con la CA raíz. Primero la cadena y al final del archivo la CA raíz. El orden es importante.

Otorgamos los permisos necesarios y nos cambiamos al usuario zimbra::

	chown -R zimbra /etc/letsencrypt
	su - zimbra

Backup del directorio Zimbra SSL
+++++++++++++++++++++++++++
::

	cp -a /opt/zimbra/ssl/zimbra /opt/zimbra/ssl/zimbra.$(date "+%Y%m%d")

Verifique su certificado comercial.
++++++++++++++++++++++++++++++++

Copie toda la carpeta Let's Encrypt con todos los archivos /etc/letsencrypt/live/$domain en /opt/zimbra/ssl/letsencrypt::

	cp /etc/letsencrypt/live/$HOSTNAME/privkey.pem /opt/zimbra/ssl/zimbra/commercial/commercial.key

Verificar el certificado SSL con zimbra
+++++++++++++++++++++++++++++++++++

Como usuario de zimbra::

	su - zimbra

	/opt/zimbra/bin/zmcertmgr verifycrt comm privkey.pem cert.pem chain.pem
	** Verifying 'cert.pem' against 'privkey.pem'
	Certificate 'cert.pem' and private key 'privkey.pem' match.
	** Verifying 'cert.pem' against 'chain.pem'
	Valid certificate chain: cert.pem: OK


Deploy el certificado SSL con zimbra
+++++++++++++++++++++++++++++++++++

Como usuario de zimbra::

	/opt/zimbra/bin/zmcertmgr deploycrt comm cert.pem chain.pem
	** Verifying 'cert.pem' against '/opt/zimbra/ssl/zimbra/commercial/commercial.key'
	Certificate 'cert.pem' and private key '/opt/zimbra/ssl/zimbra/commercial/commercial.key' match.
	** Verifying 'cert.pem' against 'chain.pem'
	Valid certificate chain: cert.pem: OK
	** Copying 'cert.pem' to '/opt/zimbra/ssl/zimbra/commercial/commercial.crt'
	** Copying 'chain.pem' to '/opt/zimbra/ssl/zimbra/commercial/commercial_ca.crt'
	** Appending ca chain 'chain.pem' to '/opt/zimbra/ssl/zimbra/commercial/commercial.crt'
	** Importing cert '/opt/zimbra/ssl/zimbra/commercial/commercial_ca.crt' as 'zcs-user-commercial_ca' into cacerts '/opt/zimbra/common/lib/jvm/java/lib/security/cacerts'
	** NOTE: restart mailboxd to use the imported certificate.
	** Saving config key 'zimbraSSLCertificate' via zmprov modifyServer mail.e-deus.cf...failed (rc=1)
	** Installing imapd certificate '/opt/zimbra/conf/imapd.crt' and key '/opt/zimbra/conf/imapd.key'
	** Copying '/opt/zimbra/ssl/zimbra/commercial/commercial.crt' to '/opt/zimbra/conf/imapd.crt'
	** Copying '/opt/zimbra/ssl/zimbra/commercial/commercial.key' to '/opt/zimbra/conf/imapd.key'
	** Creating file '/opt/zimbra/ssl/zimbra/jetty.pkcs12'
	** Creating keystore '/opt/zimbra/conf/imapd.keystore'
	** Installing ldap certificate '/opt/zimbra/conf/slapd.crt' and key '/opt/zimbra/conf/slapd.key'
	** Copying '/opt/zimbra/ssl/zimbra/commercial/commercial.crt' to '/opt/zimbra/conf/slapd.crt'
	** Copying '/opt/zimbra/ssl/zimbra/commercial/commercial.key' to '/opt/zimbra/conf/slapd.key'
	** Creating file '/opt/zimbra/ssl/zimbra/jetty.pkcs12'
	** Creating keystore '/opt/zimbra/mailboxd/etc/keystore'
	** Installing mta certificate '/opt/zimbra/conf/smtpd.crt' and key '/opt/zimbra/conf/smtpd.key'
	** Copying '/opt/zimbra/ssl/zimbra/commercial/commercial.crt' to '/opt/zimbra/conf/smtpd.crt'
	** Copying '/opt/zimbra/ssl/zimbra/commercial/commercial.key' to '/opt/zimbra/conf/smtpd.key'
	** Installing proxy certificate '/opt/zimbra/conf/nginx.crt' and key '/opt/zimbra/conf/nginx.key'
	** Copying '/opt/zimbra/ssl/zimbra/commercial/commercial.crt' to '/opt/zimbra/conf/nginx.crt'
	** Copying '/opt/zimbra/ssl/zimbra/commercial/commercial.key' to '/opt/zimbra/conf/nginx.key'
	** NOTE: restart services to use the new certificates.
	** Cleaning up 9 files from '/opt/zimbra/conf/ca'
	** Removing /opt/zimbra/conf/ca/ca.key
	** Removing /opt/zimbra/conf/ca/ca.pem
	** Removing /opt/zimbra/conf/ca/77927c8c.0
	** Removing /opt/zimbra/conf/ca/commercial_ca_1.crt
	** Removing /opt/zimbra/conf/ca/8d33f237.0
	** Removing /opt/zimbra/conf/ca/commercial_ca_2.crt
	** Removing /opt/zimbra/conf/ca/4042bcee.0
	** Removing /opt/zimbra/conf/ca/commercial_ca_3.crt
	** Removing /opt/zimbra/conf/ca/2e5ac55d.0
	** Copying CA to /opt/zimbra/conf/ca
	** Copying '/opt/zimbra/ssl/zimbra/ca/ca.key' to '/opt/zimbra/conf/ca/ca.key'
	** Copying '/opt/zimbra/ssl/zimbra/ca/ca.pem' to '/opt/zimbra/conf/ca/ca.pem'
	** Creating CA hash symlink '77927c8c.0' -> 'ca.pem'
	** Creating /opt/zimbra/conf/ca/commercial_ca_1.crt
	** Creating CA hash symlink '8d33f237.0' -> 'commercial_ca_1.crt'
	** Creating /opt/zimbra/conf/ca/commercial_ca_2.crt
	** Creating CA hash symlink '4042bcee.0' -> 'commercial_ca_2.crt'
	** Creating /opt/zimbra/conf/ca/commercial_ca_3.crt
	** Creating CA hash symlink '2e5ac55d.0' -> 'commercial_ca_3.crt'
	[zimbra@mail mail.e-deus.cf-0003]$ zmcontrol restart

Reiniciamos Zimbra
+++++++++++++++++++
::

	zmcontrol restart


Test el nuevo SSL Certificado
++++++++++++++++++++++


Test el nuevo SSL Certificado con OpenSSL
++++++++++++++++++++++
::

	echo QUIT | openssl s_client -connect e-deus.cf:443 | openssl x509 -noout -text | less

Verifying SSL certificate is not expired
+++++++++++++++++++++++++++++++++










