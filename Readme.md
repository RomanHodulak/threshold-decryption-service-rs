# Threshold decryption service

This repository implements (t, n)-threshold encryption scheme as a service.

## Features

* Split private key into `n` shares
* Define how much `t` of the shares are needed out of the total `n` to decrypt
* Encrypt a message using the public key hosted on the server
* Decrypt a message with at least `t` shares of the private key

## Known limitations

* Only one private-public key pair is supported.
* Only RSA private keys are supported for decryption.

## Requirements

Before using the libraries in this repository, make sure you have installed:

* curl 8.6.0
* Rust 1.81 nightly
* openssl 3.2.1

## Usage

### Generate private-public key pair

To obtain a new private-public keypair you can run:

```bash
ssh-keygen -P "" -t rsa -b 2048 -m pkcs8 -f key && ssh-keygen -f key.pub -m pkcs8 -e > public_key.pem
```

Note that PKCS 8 format is used, which is important for the server-side decryption using the private key.

### Generate shares of the private key

To generate shares of a secret of 5 shares with the threshold of 3, run:

```bash
cargo run -p generate_keys -- -k $PWD/private_key.pem -o $PWD/share -t 3 -s 5
```

This will split `private_key.pem` in your current working directory to `share{1,5}`.

### Run the server

To run the server locally with public key stored in `public_key.pem` and a threshold of 3, run:

```bash
cargo run -p server -- -p public_key.pem -t 3
```

### Retrieve the public key from the server

To get the public key while the server is running, use:

```bash
curl http://localhost:8080/public_key 2>/dev/null > public_key.pem
```

This will store it as `public_key.pem`

### Encrypt a message

To encrypt a message, use the previously obtained public key and encrypt a plain-text with it. For example, like so:

```bash
echo my-message | openssl rsautl -encrypt -pubin -inkey public_key.pem > cipher.bin
```

This will generate encrypted text file called `cipher.bin`

### Send encrypted message

This will store the encrypted message from a text file called `cipher.bin`. Previously added encrypted message and
shares will be cleared.

```bash
curl -X POST http://localhost:8080/send-message --data-binary "@cipher.bin"
```

### Decrypt a message

This will add a share to the set of private key shares.

After `threshold` amount of different private key shares have been submitted, the message will be decrypted.

It can be called repeatedly with the same share but never without any share.

```bash
curl -X POST http://localhost:8080/decrypt --data-binary "@share1"
```

### Full example

#### Server-side

```bash
# Generate private-public key pairs
ssh-keygen -P "" -t rsa -b 2048 -m pkcs8 -f key && ssh-keygen -f key.pub -m pkcs8 -e > public_key.pem

# Generate shares of private key
cargo run -p generate_keys -- -k $PWD/key -o $PWD/share

# Run server
cargo run -p server -- -p public_key.pem -t 3
```

#### Client-side

```bash
# Encrypt a message
echo hello world | openssl rsautl -encrypt -pubin -inkey <(curl http://localhost:8080/public_key 2>/dev/null) > cipher.bin

# Send an encrypted message
curl -X POST http://localhost:8080/send-message --data-binary "@cipher.bin"

# Decrypt the message
curl -X POST http://localhost:8080/decrypt --data-binary "@share1"
curl -X POST http://localhost:8080/decrypt --data-binary "@share2"
curl -X POST http://localhost:8080/decrypt --data-binary "@share3"
```

## Implementation report

If you're interested in reasoning behind the design choices made, see [Report.md](./Report.md).
