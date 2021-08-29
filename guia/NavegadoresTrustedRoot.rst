
Para los Navegador (Firefox, Chrome, Chromium,…)
=======================================

Los navegadores web como Firefox, Chromium, Google Chrome e incluso los clientes de correo electrónico como Mozilla Thunderbird no hacen uso del almacén de confianza del sistema operativo, sino que utilizan su propio almacén de confianza de certificados. Estos almacenes de confianza son archivos en el directorio de usuario, llamados "**cert8.db**" y "**cert9.db**" (para las versiones más recientes). Puede modificar los archivos del almacén de confianza utilizando la herramienta "certutil". 


Previo debe instalar la trusted root dentro del server, ver el siguiente documento:

https://github.com/cgomeznt/Certificados/blob/master/guia/addcertificaterootserver.rst

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
