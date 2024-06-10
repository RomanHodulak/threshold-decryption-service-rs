# Threshold decryption service

This repository implements (t, n)-threshold encryption scheme as a service.

## Features

* Split private key into `n` shares
* Define how much `t` of the shares are needed out of the total `n` to decrypt
* Encrypt a message using the public key hosted on the server
* Decrypt a message with at least `t` shares of the private key

## Known limitations

* Only one private-public key pair is supported.

## Requirements

Before using the libraries in this repository, make sure you have installed:

* curl 8.6.0
* Rust 1.81 nightly

## Usage

### Generate shares of the private key

To generate shares of a secret of 5 shares with the threshold of 3, run:

```bash
cargo run -p generate_keys -- -k $PWD/private_key.pem -o $PWD/share_ -t 3 -s 5
```

This will split `private_key.pem` in your current working directory to `share_{1,5}.pem`.

### Run the server

To run the server locally with public key stored in `public_key.pem` and a threshold of 3, run:

```bash
cargo run -p server -- -p public_key.pem -t 3
```

### Retrieve the public key from the server

To get the public key while the server is running locally, use:

```bash
curl http://localhost:8080/public_key
```
