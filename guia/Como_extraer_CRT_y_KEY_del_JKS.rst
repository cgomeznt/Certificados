Como extraer un CRT y KEY de un JKS
======================================

Para consultar el JKS suministrado por Criptografia::

	keytool -list -v -storepass 'Get-d6.3+6d' -keystore srv-vccs-subversiondes.credicard.com.ve.jks

Como exportar el CRT y el KEY a un formato P12::

	keytool -importkeystore \
			 -srckeystore srv-vccs-subversiondes.credicard.com.ve.jks \
			 -destkeystore keystore.p12 \
			 -deststoretype PKCS12 \
			 -srcalias '2305182747862202823cn=srv-vccs-subversiondes.credicard.com.ve, ou=criptografia, o=consorcio credicard c.a, l=caracas, st=distrito capital, c=vecn=consorcio credicard ca, ou=criptografia, o=consorcio credicard ca, l=caracas, st=distrito capital, c=ve' \
			 -deststorepass 'Get-d6.3+6d' \
			 -destkeypass 'Get-d6.3+6d'

Como consultar el P12 y estar seguros que se exporto bien::

	openssl pkcs12 -info -in keystore.p12

Como exportar el CRT del P12::

	openssl pkcs12 -in keystore.p12  -nokeys -out cert.crt

Como exportar el KEY del P12::

	openssl pkcs12 -in keystore.p12  -nodes -nocerts -out cert.key
	
Como consultar el CRT::

	openssl pkcs12 -in keystore.p12  -nokeys -out cert.crt

Como Consultar el KEY::

	openssl pkcs12 -in keystore.p12  -nodes -nocerts -out cert.key