Agregar certificados raíz en los depósitos de confianza de los servidores
=========================================================================

En la mayoría de los casos, no es recomendable ejecutar una CA (autoridad de certificación) propia. Pero hay excepciones: si desea asegurar los servicios internos de su empresa, puede ser necesario usar su propia CA. veremos cómo importar la nueva CA a las PC/Servers con Linux y Windows y a todos los principales navegadores web.

Si desea enviar o recibir mensajes firmados por las autoridades de la raíz y las autoridades no están instaladas en el servidor, debe agregar manualmente un certificado raíz de confianza.


**IMPORTANTE** Después de ejecutar los siguientes pasos, la nueva CA sera conocida por las utilidades del sistema como curl, get y otros servicios. Desafortunadamente, **esto no afecta a la mayoría de los navegadores web como Mozilla Firefox o Google Chrome.**



Esta son las rutas absolutas para los certificados publicos CA
++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++

::

	"/etc/ssl/certs/ca-certificates.crt",                // Debian/Ubuntu/Gentoo etc.
	"/etc/pki/tls/certs/ca-bundle.crt",                  // Fedora/RHEL 6
	"/etc/ssl/ca-bundle.pem",                            // OpenSUSE
	"/etc/pki/tls/cacert.pem",                           // OpenELEC
	"/etc/pki/ca-trust/extracted/pem/tls-ca-bundle.pem", // CentOS/RHEL 7
	"/etc/ssl/cert.pem",                                 // Alpine Linux

Tambien::

	"/etc/ssl/certs",               // SLES10/SLES11, https://golang.org/issue/12139
	"/system/etc/security/cacerts", // Android
	"/usr/local/share/certs",       // FreeBSD
	"/etc/pki/tls/certs",           // Fedora/RHEL
	"/etc/openssl/certs",           // NetBSD
	"/var/ssl/certs",               // AIX



Mac OS X
+++++++++++++

Añadir

Utilice el comando:::

	sudo security add-trusted-cert -d -r trustRoot -k /Library/Keychains/System.keychain ~/new-root-certificate.crt

 

Quitar

Utilice el comando::

	sudo security delete-certificate -c "<name of existing certificate>"

 

Windows
++++++++++++++++
 

Añadir

Utilice el comando::

	certutil -addstore -f "ROOT" new-root-certificate.crt

 

Quitar

Utilice el comando::

	certutil -delstore "ROOT" serial-number-hex

 

Linux (Ubuntu, Debian)
++++++++++++++++++++++++++
 

Añadir

Copiar su CA a dir /usr/local/share/ca-certificates/
 

Utilice el comando::

	cp CA_empresa.crt /usr/local/share/ca-certificates/

Agreguelo en el archivo esto "mozilla/CA_empresa.crt"::

	vi /etc/ca-certificates.conf
	[...]
	mozilla/CA_empresa.crt
	[...]	

 
Actualización de la tienda de CA::

	update-ca-certificates
	1 added, 0 removed; done.
	Running hooks in /etc/ca-certificates/update.d...

	Replacing debian:CA_empresa.pem
	done.
	done.


 

Quitar

Retire su CA.


Actualización de la tienda de CA::

	update-ca-certificates --fresh

 

Linux (CentOs 6 / 7)
+++++++++++++++++

 

Añadir

Instalar el paquete de certificados de ca::

	yum install ca-certificates

 

Habilitar la característica de configuración dinámica de CA::

	update-ca-trust force-enable

 

Agregar como un nuevo archivo en /etc/pki/ca-trust/source/anchors/::

	cp foo.crt /etc/pki/ca-trust/source/anchors/

 

Utilice el comando::

	update-ca-trust 


 

Linux (CentOs 5)
+++++++++++++++++++

 

Añadir

 

Anexar el certificado de confianza para presentar /etc/pki/tls/certs/ca-bundle.crt::

	cat foo.crt >> /etc/pki/tls/certs/ca-bundle.crt




Para los Navegador (Firefox, Chrome, Chromium,…)
++++++++++++++++++++++++++++++++++++++++++++++++++

Los navegadores web como Firefox, Chromium, Google Chrome e incluso los clientes de correo electrónico como Mozilla Thunderbird no hacen uso del almacén de confianza del sistema operativo, sino que utilizan su propio almacén de confianza de certificados. Estos almacenes de confianza son archivos en el directorio de usuario, llamados "**cert8.db**" y "**cert9.db**" (para las versiones más recientes). Puede modificar los archivos del almacén de confianza utilizando la herramienta "certutil". 

Para instalar certutil, ejecute el siguiente comando apt::

	# apt-get install libnss3-tools


Con este pequeño script importa el nuevo certificado raíz en las bases de datos del almacén de confianza de "**cert8.db**" y "**cert9.db**"::

	#!/bin/bash

	### Script installs root.cert.pem to certificate trust store of applications using NSS
	### (e.g. Firefox, Thunderbird, Chromium)
	### Mozilla uses cert8, Chromium and Chrome use cert9

	certfile="/path/CA_empresa.crt"
	certname="Personal empresa C.A."


	###
	### For cert8 (legacy - DBM)
	###

	for certDB in $(find ~/ -name "cert8.db")
	do
	    certdir=$(dirname ${certDB});
	    certutil -A -n "${certname}" -t "TCu,Cu,Tu" -i ${certfile} -d dbm:${certdir}
	done


	###
	### For cert9 (SQL)
	###

	for certDB in $(find ~/ -name "cert9.db")
	do
	    certdir=$(dirname ${certDB});
	    certutil -A -n "${certname}" -t "TCu,Cu,Tu" -i ${certfile} -d sql:${certdir}
	done


Después de la ejecución de este script, su CA raíz debe ser conocida por Firefox, Chrome, Chromium, Vivaldy y otros navegadores.


Una manera de realizar las pruebas es con el curl. Si aun no instala el certificado de la CA, se vera algo como esto::

	curl https://monitoreo.empresa.local

	curl: (60) Peer's certificate issuer has been marked as not trusted by the user.
	More details here: http://curl.haxx.se/docs/sslcerts.html

	curl performs SSL certificate verification by default, using a "bundle"
	 of Certificate Authority (CA) public keys (CA certs). If the default
	 bundle file isn't adequate, you can specify an alternate file
	 using the --cacert option.
	If this HTTPS server uses a certificate signed by a CA represented in
	 the bundle, the certificate verification probably failed due to a
	 problem with the certificate (it might be expired, or the name might
	 not match the domain name in the URL).
	If you'd like to turn off curl's verification of the certificate, use
	 the -k (or --insecure) option.

Después de instalar el certificado de la CA, ahí si podrá ver todo bien::

	curl https://monitoreo.consis.local
	<html>
	  <head>
		<title>monitoreo.empresa.local</title>
	  </head>
	  <body>
		<h1>Felicitaciones, se creo el Virtual Host de monitoreo.empresa.local</h1>
	  </body>
	</html>

Y por supuesto desde un navegador debe abrir y mostrar la pagina sin ningún tipo de Warning o Error en el certificado

	.. figure:: ../images/04.png
