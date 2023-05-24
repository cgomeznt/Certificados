


Copiar el certificado de la CA en “/etc/pki/ca-trust/source/anchors/”

	cp /home/cgomez/ca.crt  /etc/pki/ca-trust/source/anchors/ca-2026.crt



Luego ejecutamos este comando

	update-ca-trust


Pero es muy buena práctica al comando curl agregarle el siguiente parámetro para indicarle en donde está la CA

	curl --cacert /etc/httpd/conf.d/certs/ca.crt -s --head https://direccion_de_url.html
