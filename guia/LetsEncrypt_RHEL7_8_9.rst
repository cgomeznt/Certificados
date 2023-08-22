Certificados Digitales Letsencrypt Standalone RHEL 7/8/9 y Debian 11
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

Instalando **certbot** en **Debian 11**
++++++++++++++++++++++++++++++
::

	apt install snapd
	
	snap install core
	
	apt-get remove certbot
	
	snap install --classic certbot
	
	ln -s /snap/bin/certbot /usr/bin/certbot

Creando el certificado firmado por letsencrypt
++++++++++++++++++++++++++++++++++++++++++++++
::

	# certbot certonly -d e-deus.online -d mail.e-deus.online -m cgomeznt@gmail.com --standalone
	Saving debug log to /var/log/letsencrypt/letsencrypt.log
	Requesting a certificate for mail.e-deus.online and e-deus.online

	Successfully received certificate.
	Certificate is saved at: /etc/letsencrypt/live/mail.e-deus.online/fullchain.pem
	Key is saved at:         /etc/letsencrypt/live/mail.e-deus.online/privkey.pem
	This certificate expires on 2021-10-05.
	These files will be updated when the certificate renews.
	Certbot has set up a scheduled task to automatically renew this certificate in the background.

Si necesita tener varios nombres de host en el mismo SSL, entonces un Multi-SAN, SSL, por favor ejecute en su lugar, donde -d son sus dominios:::

	certbot  certonly  -d mail.e-deus.online -d e-deus.online -m cgomeznt@e-deus.online--standalone

¿Dónde están los archivos de certificado SSL?
++++++++++++++++++++++++++++++

Puede encontrar todos sus archivos en /etc/letsencrypt/live/$domain, donde $domain es el fqdn que utilizó durante el proceso::

	ls -l /etc/letsencrypt/live/mail.e-deus.online/
	total 4
	lrwxrwxrwx 1 zimbra root  43 jul  7 16:56 cert.pem -> ../../archive/mail.e-deus.online/cert1.pem
	lrwxrwxrwx 1 zimbra root  44 jul  7 16:56 chain.pem -> ../../archive/mail.e-deus.online/chain1.pem
	lrwxrwxrwx 1 zimbra root  48 jul  7 16:56 fullchain.pem -> ../../archive/mail.e-deus.online/fullchain1.pem
	lrwxrwxrwx 1 zimbra root  46 jul  7 16:56 privkey.pem -> ../../archive/mail.e-deus.online/privkey1.pem
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

Otorgamos los permisos necesarios, si lo requiere::

	chown -R zimbra /etc/letsencrypt

Test el nuevo SSL Certificado con OpenSSL
++++++++++++++++++++++
::

	echo QUIT | openssl s_client -connect e-deus.online:443 | openssl x509 -noout -text | less









