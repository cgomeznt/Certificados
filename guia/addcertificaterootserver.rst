Agregar certificados raíz en los depositos de confianza de los servidores
=========================================================================


Si desea enviar o recibir mensajes firmados por las autoridades de la raíz y las autoridades no están instaladas en el servidor, debe agregar manualmente un certificado raíz de confianza.

Utilice los pasos siguientes para agregar o quitar certificados raíz de confianza de un servidor.



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


Actualización de la tienda de CA:;

	update-ca-certificates --fresh

 

Linux (CentOs 6)
+++++++++++++++++

 

Añadir

 

Instalar el paquete de certificados de ca:

yum install ca-certificates

 

Habilitar la característica de configuración dinámica de CA:

update-ca-trust force-enable

 

Agregar como un nuevo archivo en /etc/pki/ca-trust/source/anchors /:

cp foo.crt /etc/pki/ca-trust/source/anchors/

 

Utilice el comando:

update-ca-trust extract


 

Linux (CentOs 5)
+++++++++++++++++++

 

Añadir

 

Anexar el certificado de confianza para presentar /etc/pki/tls/certs/ca-bundle.crt

cat foo.crt >> /etc/pki/tls/certs/ca-bundle.crt

