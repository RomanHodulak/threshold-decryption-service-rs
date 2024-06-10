#!/bin/bash

# Generate private-public key pairs
ssh-keygen -P "" -t rsa -b 2048 -m pkcs8 -f key && ssh-keygen -f key.pub -m pkcs8 -e > public_key.pem

# Generate shares of private key
cargo run -p generate_keys -- -k $PWD/key -o $PWD/share

# Encrypt a message
echo "$1" | openssl rsautl -encrypt -pubin -inkey <(curl http://localhost:8080/public_key 2>/dev/null) > cipher.txt

# Send message for decryption
curl -X POST http://localhost:8080/send-message --data-binary "@cipher.txt"

# Decrypt the message
curl -X POST http://localhost:8080/decrypt --data-binary "@share1"
curl -X POST http://localhost:8080/decrypt --data-binary "@share2"
curl -X POST http://localhost:8080/decrypt --data-binary "@share3"
