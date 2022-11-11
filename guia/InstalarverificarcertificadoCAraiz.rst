2 formas de instalar y verificar el certificado de CA raíz en Linux
====================================================================

Tenemos dos métodos para usar update-ca-trust o trust ancla para agregar un certificado de CA en Linux.

Necesitamos instalar el paquete ca-certificates primero con el comando yum install ca-certificates.

Entender los certificado Root CA
++++++++++++++++++++++++++++++++++++++

Los certificados SSL operan en una estructura llamada cadena de certificados: 
una red de certificados que comienza en la empresa emisora ​​del certificado, también conocida como autoridad certificadora (CA). 
Estos certificados constan de certificados raíz, certificados intermedios y certificados de hoja (servidor).

En cuanto a los certificados Root CA, estos son certificados autofirmados por sus respectivas CA (ya que tienen la autoridad para hacerlo). 
Todos los certificados SSL válidos se encuentran bajo un certificado de CA raíz, ya que son partes de confianza.



 

¿Necesitamos instalar el certificado CA?
++++++++++++++++++++++++++++++++++++++++++++++

Por lo general, no necesitamos instalar un certificado de CA raíz, ya que están incluidos en las tiendas de confianza de los navegadores web e 
incluso están preinstalados en algunos sistemas operativos. 
Esto permite que nuestra computadora pueda saber si un certificado no es válido o no, porque si su certificado raíz no está en su lista de CA raíz confiable, 
entonces nos advertirá que el certificado no es confiable.

Uso de update-ca-trust para instalar un certificado de CA
+++++++++++++++++++++++++++++++++++++++++++++++++++++++++

Copie el certificado de CA en el directorio **/etc/pki/ca-trust/source/anchors/**::

	# cp rapidSSL-ca.crt /etc/pki/ca-trust/source/anchors/
	
Integrar el certificado de CA a la lista de CA de confianza::

	# update-ca-trust
	
Verifique el certificado SSL::

	# openssl verificar server.crt 
	server.crt: OK
 

Uso de trust anchor para agregar un certificado de CA. 
+++++++++++++++++++++++++++++++++++++++++++++++++++++


Ejecute el trust anchor –almacenar especificando el certificado de CA::

	# trust anchor –store ca.crt

Consulte la lista de CA de confianza:: 

	# trust list
	pkcs11:id=%53%ca%17%59%fc%6b%c0%03%21%2f%1a
	type: certificate
	label: RapidSSL RSA CA 2018
	trust: anchor
	category: authority
	..snip..


verificar el certificado del servidor::

	# openssl verify server.crt
	server.crt: OK

Si queremos eliminar el certificado de CA
+++++++++++++++++++++++++++++++++++++++++++

Ejecute el ancla de confianza –remove de la siguiente manera::

	# trust anchor –remove pkcs11:id=%53%ca%17%59
	
or::
	
	# trust anchor –remove /etc/pki/ca-trust/source/RapidSSL_RSA_CA_2018.p11-kit

Listar todos los certificados de CA en Linux
+++++++++++++++++++++++++++++++++++++++++++

Una vez que se agrega el certificado ca, el certificado está disponible a través del árbol /etc/pki/ca-trust/extracted::

	$ ls /etc/pki/ca-trust/extracted
	edk2 java openssl pem LÉAME

Las aplicaciones que consultan este directorio para verificar los certificados pueden utilizar cualquiera de los formatos proporcionados. 
El comando de actualización maneja las copias, conversiones y consolidación para los diferentes formatos. 
La página del manual de update-ca-trust tiene más información sobre la estructura del directorio, los formatos y las formas en que se accede a los certificados.

Tenemos una forma rápida de enumerar todos los sujetos del certificado en el paquete con los siguientes comandos awk y openssl::

	$  awk -v cmd='openssl x509 -noout -subject' '/BEGIN/{close(cmd)};{print | cmd}' < /etc/pki/ca-trust/extracted/pem/tls-ca-bundle.pem




 
