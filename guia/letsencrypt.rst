Letsencrypt Almalinux 8 Apache
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

Instalando **certbot** con dnf
++++++++++++++++++++++++++++++
::

	dnf -y install epel-release
	dnf install certbot python3-certbot-apache mod_ssl


Instalando **certbot** con snap
++++++++++++++++++++++++++++++
::

	yum install snapd

	systemctl enable --now snapd.socket

	ln -s /var/lib/snapd/snap /snap

	snap install core; sudo snap refresh core

	snap install --classic certbot

	ln -s /snap/bin/certbot /usr/bin/certbot

instalar el httpd  y crear el virtual host
+++++++++++++++++++++++++++++++++++++++++++++
::

	dnf install httpd*

	systemctl enable httpd

	systemctl start httpd

	vi /etc/httpd/conf.d/e-deus.cf.conf
	<VirtualHost *:80>
	ServerName e-deus.cf
	ServerAlias e-deus.cf
	DocumentRoot /var/www/html
	</VirtualHost>

Creamos un index para verificar el apache este bien::

	vi /var/www/html/index.html
	<html>
	  <head>
		<title>e-deus.cf</title>
	  </head>
	  <body>
		<h1>Felicitaciones, se creo el Virtual Host de e-deus.cf</h1>
	  </body>
	</html>

ahora vamos a instalar el certificado con lestsencript y crear un virtual host.

Creando el certificado firmado por letsencrypt
++++++++++++++++++++++++++++++++++++++++++++++
debemos bajar el apache::

	systemctl stop httpd

Descargamos el tools de letsencrypt::

	git clone https://github.com/letsencrypt/letsencrypt
	cd letsencrypt/letsencrypt-auto-source/

Ahora creamos el certificado la llave con la ayuda de certbot::

	# certbot certonly -d e-deus.cf -m cgomeznt@gmail.com --standalone
	Saving debug log to /var/log/letsencrypt/letsencrypt.log
	Requesting a certificate for e-deus.cf

	- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
	Could not bind TCP port 80 because it is already in use by another process on
	this system (such as a web server). Please stop the program in question and then
	try again.
	- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
	(R)etry/(C)ancel: R

	Successfully received certificate.
	Certificate is saved at: /etc/letsencrypt/live/e-deus.cf/fullchain.pem
	Key is saved at:         /etc/letsencrypt/live/e-deus.cf/privkey.pem
	This certificate expires on 2023-01-30.
	These files will be updated when the certificate renews.
	Certbot has set up a scheduled task to automatically renew this certificate in the background.

	- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
	If you like Certbot, please consider supporting our work by:
	 * Donating to ISRG / Let's Encrypt:   https://letsencrypt.org/donate
	 * Donating to EFF:                    https://eff.org/donate-le
	- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -


Si necesita tener varios nombres de host en el mismo SSL, entonces un Multi-SAN, SSL, por favor ejecute en su lugar, donde -d son sus dominios:::

	certbot  certonly  -d mail.e-deus.cf -d e-deus.cf -m cgomeznt@e-deus.cf--standalone

¿Dónde están los archivos de certificado SSL?
++++++++++++++++++++++++++++++

Puede encontrar todos sus archivos en /etc/letsencrypt/live/$domain, donde $domain es el fqdn que utilizó durante el proceso::

	ls -l /etc/letsencrypt/live/e-deus.cf/
	total 4
	lrwxrwxrwx 1 root root  36 Nov  1 15:56 privkey.pem -> ../../archive/e-deus.cf/privkey1.pem
	lrwxrwxrwx 1 root root  38 Nov  1 15:56 fullchain.pem -> ../../archive/e-deus.cf/fullchain1.pem
	lrwxrwxrwx 1 root root  34 Nov  1 15:56 chain.pem -> ../../archive/e-deus.cf/chain1.pem
	lrwxrwxrwx 1 root root  33 Nov  1 15:56 cert.pem -> ../../archive/e-deus.cf/cert1.pem
	-rw-r--r-- 1 root root 692 Nov  1 15:56 README


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
	-----END CERTIFICATE-----">> /etc/letsencrypt/live/e-deus.cf/chain.pem

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

Otorgamos los permisos si es necesario (Opcional)::

	chown -R apache /etc/letsencrypt



Creamos el virtual host e iniciamos el apache::

	vi /etc/httpd/conf.d/e-deus.cf-SSL.conf
	<VirtualHost *:443>
	ServerName e-deus.cf
	ServerAlias e-deus.cf
	DocumentRoot /var/www/html
	SSLEngine on
	SSLCertificateFile /etc/letsencrypt/live/e-deus.cf/cert.pem
	SSLCertificateKeyFile /etc/letsencrypt/live/e-deus.cf/privkey.pem
	SSLCertificateChainFile /etc/letsencrypt/live/e-deus.cf/chain.pem
	</VirtualHost>


Test el nuevo SSL Certificado
++++++++++++++++++++++

https://e-deus.cf

Test el nuevo SSL Certificado con OpenSSL
++++++++++++++++++++++
::

	echo QUIT | openssl s_client -connect e-deus.cf:443 | openssl x509 -noout -text | less

Verifying SSL certificate is not expired
+++++++++++++++++++++++++++++++++



**NOTA** recordemos que estos certificados duran  tres 3 meses






