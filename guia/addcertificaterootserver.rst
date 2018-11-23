Agregar certificados raíz en los depósitos de confianza de los servidores
=========================================================================

En la mayoría de los casos, no es recomendable ejecutar una CA (autoridad de certificación) propia. Pero hay excepciones: si desea asegurar los servicios internos de su empresa, puede ser necesario usar su propia CA. veremos cómo importar la nueva CA a las PC/Servers con Linux y Windows y a todos los principales navegadores web.

Si desea enviar o recibir mensajes firmados por las autoridades de la raíz y las autoridades no están instaladas en el servidor, debe agregar manualmente un certificado raíz de confianza.


**IMPORTANTE** Después de ejecutar los siguientes pasos, la nueva CA sera conocida por las utilidades del sistema como curl, get y otros servicios. Desafortunadamente, **esto no afecta a la mayoría de los navegadores web como Mozilla Firefox o Google Chrome.**


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

Copiar su CA a dir /usr/local/share/ca-certificates/mozilla
 

Utilice el comando::

	cp CA_empresa.crt /usr/local/share/ca-certificates/mozilla/CA_empresa.crt

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

 

Linux (CentOs 6)
+++++++++++++++++

 

Añadir

Instalar el paquete de certificados de ca::

	yum install ca-certificates

 

Habilitar la característica de configuración dinámica de CA::

	update-ca-trust force-enable

 

Agregar como un nuevo archivo en /etc/pki/ca-trust/source/anchors /::

	cp foo.crt /etc/pki/ca-trust/source/anchors/

 

Utilice el comando::

	update-ca-trust extract


 

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
