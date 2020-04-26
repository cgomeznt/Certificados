
En este ejercicio se tiene una Entidad Certificadora CA de la Empresas

Editamos el archivo de configuracion.::

	# vi conf/lab01.cnf

Generar la LLave .key y se genera el request .csr::

	# openssl req -newkey rsa:2048 -nodes -keyout keyservice/lab01.key -out request/lab01.csr -subj "/C=VE/ST=DC/L=Caracas/O=PERSONAL/OU=TI/CN=lab01"

Consultamos la llave y el request::

	# openssl req -text -noout -in /opt/CA/request/lab01.csr
	# openssl rsa -in lab01.key -check
	
Generar el certificado .crt firmado por la CA de la Empresa::

	# openssl x509 -req -days 185 -extfile conf/lab01.cnf -extensions v3_req -CA certs/CA_cursoinfraestructura.crt -CAkey private/CA_cursoinfraestructura.key -CAserial ca.srl -CAcreateserial -in request/lab01.csr -out newcerts/lab01.crt


Copiamos la llave y el certificado, tambien el certificado de la CA al directorio de trabajo.::

	# mkdir workdir
	# cd workdir
	# cp /opt/CA/newcerts/lab01.crt /opt/CA/keyservice/lab01.key /opt/CA/certs/CA_cursoinfraestructura.crt .

	
Consultamos los certificados::

	# openssl x509 -in lab01.crt -noout -text
	# openssl x509 -in CA_cursoinfraestructura.crt -noout -text

Creamos un certificado .pem::

	# cat lab01.key lab01.crt > lab01.pem

Consultamos el certificado .pem::

	# openssl x509 -in lab01.pem -inform pem -noout -text
	
Verificamos el certificado .pem::

	# openssl verify -CAfile CA_cursoinfraestructura.crt lab01.pem 
	
Creamos un certificado .p12::

	# openssl pkcs12 -export -name cursoinfraestructura.com.ve -in lab01.crt -inkey lab01.key -out lab01.p12

Consultamos el certificado .p12::

	# openssl pkcs12 -info -in lab01.p12

Creamos el almacen de claves Keystore agregando el p12 anteriormente::

	# keytool -importkeystore -destkeystore Mykeystore.jks -srckeystore lab01.p12 -srcstoretype pkcs12 -alias cursoinfraestructura.com.ve

Consultamos el keystore::

	# keytool -list -v -keystore Mykeystore.jks
	
Agregamos la CA dentro del keystore::

	# keytool -import -alias CA-cursoinfraestrutura -file CA_cursoinfraestructura.crt -keystore Mykeystore.jks

Consultamos el keystore::

	# keytool -list -v -keystore Mykeystore.jks -storepass Venezuela21

Agregamos el certificado dentro del keystore::

	# keytool -import -alias cert-cursoinfraestrutura -file lab01.crt -keystore Mykeystore.jks

Consultamos el keystore::

	# keytool -list -v -keystore Mykeystore.jks -storepass Venezuela21

	
Para instalar el certificado de la CA en CentOS 7::

	# cp CA_cursoinfraestructura.crt /etc/pki/ca-trust/source/anchors/
	# update-ca-trust

Para instalar en el certificado de la CA en Windows::

	win+r certmgr.msc
	En el marco izquierdo, expanda Entidades de Certificacion raíz de confianza, haga clic con el botón derecho en Certificados y seleccione Todas las tareas >Importar, selecciones CA_cursoinfraestructura.crt

Con lo anterior el Internet Explorer y Google utilizaran dicha CA, pero Firefox NO.

En Firefox hay que ir a Herramientas -> Opciones -> Avanzado -> Certificados -> Ver certificados -> Importar y una vez allí importar el archivo CA_cursoinfraestructura.crt
	


	