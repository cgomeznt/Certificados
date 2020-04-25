Importar un Certificado de una CA en el cacerts de la JDK
===========================================================

Es habitual encontrarnos en situaciones donde una empresa tiene su propia autoridad de certificación (Certificate authority o CA) con la que firma los certificados de sus servidores o bien nos encontramos con servicios REST o web services de sites cuyo certificado esta firmado por una CA que no la tenemos en nuestro almacén de CAs de confianza.
Ver este link 
.. _Crear una Entidad certificadora CA en Centos 7 https://github.com/cgomeznt/Certificados/blob/master/guia/cacentos7.rst

En estos casos para que nuestras aplicaciones puedan establecer una conexión segura a estos servicios, necesitamos incorporar el certificado de la CA en nuestro almacén de CAs de confianza.

De esta forma, siempre y cuando nos encontremos con un certificado firmado por cualquiera de esas nuevas CAs, confiaremos automáticamente en ellos.

Como sabemos una CA firma los certificados que emite con su clave privada y es mediante el certificado de la CA con lo que podemos verificar la firma. Esta firma va incluida en el certificado emitido. Por ese motivo incorporamos el certificado de la CA entre los que confiamos.

En la JDK el almacen de CAs de confianza lo vamos a encontrar en::

	$JAVA_HOME/jre/lib/security/cacerts
	
Para incorporar el Certificado de la CA en el cacerts necesitamos importarlo.

Importar un certificado de una CA en nuestro almacén de CAs de confianza o cacerts::

	$JAVA_HOME/bin/keytool -import -alias ca-cursoinfraestructura -file /opt/CA/certs/CA_cursoinfraestructura.crt -keystore $JAVA_HOME/jre/lib/security/cacerts
	
La contraseña por defecto del cacerts es: changeit.

Este comando nos pedirá confirmación de que queremos importar este nuevo certificado en el almacén de CAs. le damos yes.

Importar un certificado de una CA en un almacén de CAs de confianza::

	$JAVA_HOME/bin/keytool -import -trustcacerts -alias ca-cursoinfraestructur -file /opt/CA/certs/CA_cursoinfraestructura.crt -keystore cacerts_app

-trustcacerts: Se utiliza cuando el -keystore no es el cacerts, y queremos que al importar el certificado, keytool tenga en cuenta los Certificados de las CAs que tenemos en el fichero cacerts


Excepción habitual que pretendemos evitar al incorpotar el Certificado de la CA

Cuando accedemos a un servicio por conexión segura y el certificado es autofirmado o no tenemos el certificado de la CA que lo firma en nuestro cacerts, obtendremos la siguiente excepción::

	Exception in thread "main" javax.xml.ws.WebServiceException: Fallo al acceder al WSDL en: https://localhost/myservice. Ha fallado con: 
		sun.security.validator.ValidatorException: PKIX path building failed: sun.security.provider.certpath.SunCertPathBuilderException: unable to find valid certification path to requested target.
		at com.sun.xml.internal.ws.wsdl.parser.RuntimeWSDLParser.tryWithMex(RuntimeWSDLParser.java:250)
		at com.sun.xml.internal.ws.wsdl.parser.RuntimeWSDLParser.parse(RuntimeWSDLParser.java:231)
		at com.sun.xml.internal.ws.wsdl.parser.RuntimeWSDLParser.parse(RuntimeWSDLParser.java:194)
		at com.sun.xml.internal.ws.wsdl.parser.RuntimeWSDLParser.parse(RuntimeWSDLParser.java:163)
		...
	Caused by: javax.net.ssl.SSLHandshakeException: sun.security.validator.ValidatorException: PKIX path building failed: sun.security.provider.certpath.SunCertPathBuilderException: unable to find valid certification path to requested target
		at sun.security.ssl.Alerts.getSSLException(Alerts.java:192)
		at sun.security.ssl.SSLSocketImpl.fatal(SSLSocketImpl.java:1937)
		at sun.security.ssl.Handshaker.fatalSE(Handshaker.java:302)
		at sun.security.ssl.Handshaker.fatalSE(Handshaker.java:296)