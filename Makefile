keygen:
	openssl ecparam -name prime256v1 -genkey -noout -out sec1.pem
	openssl pkcs8 -topk8 -nocrypt -in sec1.pem -out private.pem
	openssl ec -in private.pem -pubout -out public.pem
	rm sec1.pem