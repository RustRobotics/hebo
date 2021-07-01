
root-ca:
	openssl req -new -x509 -days 3650 -extensions v3_ca -keyout ca.key -out ca.cert

step1-keypair:
	#passphase: oohah0Miedoo
	openssl genrsa -des3 -out ca.key 2048

step2-certificate:
	openssl req -new -x509 -days 1826 -key ca.key -out ca.crt

step3-server-keypair:
	openssl genrsa -out server.key 2048

step4-server-certificate:
	openssl req -new -out server.csr -key server.key


step5-sign-server:
	openssl x509 -req -in server.csr -CA ca.crt -CAkey ca.key -CAcreateserial -out server.crt -days 360

step7-copy-server-files:
	mkdir -pv server
	cp -v ca.crt server.crt server.key server/

step8-copy-client-files:
	mkdir -pv client
	cp -v ca.crt client/
