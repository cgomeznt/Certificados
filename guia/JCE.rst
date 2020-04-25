Instalar Java Cryptography Extension (JCE) Unlimited Strength Jurisdiction Policy File
=======================================================================================

Java Cryptography Extensión (JCE) es un plugin del JRE de Java con funciones criptográficas que son necesarias para que determinados programas se puedan ejecutar. No viene instalado por defecto y es necesario instalarlo a parte.

Descargar la extensión. Accedemos a la página de Oracle,https://www.oracle.com/java/technologies/javase-jce8-downloads.html aceptamos los términos y **descargamos el ZIP**.

Una vez descargada la app, extraemos los archivos 

	# unzip jce_policy-8.zip
	Archive:  jce_policy-8.zip
	   creating: UnlimitedJCEPolicyJDK8/
	  inflating: UnlimitedJCEPolicyJDK8/local_policy.jar
	  inflating: UnlimitedJCEPolicyJDK8/README.txt
	  inflating: UnlimitedJCEPolicyJDK8/US_export_policy.jar
  
Listamos los archivos::

	# ls UnlimitedJCEPolicyJDK8/
	local_policy.jar  README.txt  US_export_policy.jar

Copiamos a los correspondientes directorios, pero antes respaldamos los originales si existieran::

	# cp UnlimitedJCEPolicyJDK8/*.jar /opt/jdk1.8.0_251/jre/lib/security/

Listamos los archivos copiados::

	# ls -l *jar
	-rw-r--r-- 1 root root 3035 Apr 25 16:34 local_policy.jar
	-rw-r--r-- 1 root root 3023 Apr 25 16:34 US_export_policy.jar

