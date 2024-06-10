# Implementation report

I started with creating the repository of two binary crates. One binary crate will serve the purpose of creating `n`
shares with threshold `t` for the (`t`, `n`)-threshold scheme.

To implement the (`t`, `n`)-threshold scheme, I used the Shamir implementation because it is one of the fastest and any
individual share does not reveal any information about the secret. I've used
the `shamir` crate. This crate is very minimalistic and provides a pure rust implementation of the Shamir's Secret
Separation algorithm. I considered some alternatives, because this one does not seem to be very popular and has not been
maintained for a couple of years, but compared to others, this one seemed good enough. Perhaps it is because it is so
minimalistic and simple that it does not need a lot of maintenance. Upon further reviewing the crate it is well covered
with tests.

First, I implemented the crate for share generation as it is a prerequisite for the server to have something to work
with. I used the `clap` crate to provide a simple to implement and familiar to users command-line interface and defined
several parameters that you can use to tweak the share generation. You may choose the number of shares, the threshold
value, the file to generate the shares for and the output path and file-name prefix for the generated share files.

As a next step I implemented the `server` crate. For the web server purposes I have used the `actix-web` crate, after
some considerations. What I mostly was interested about was performance and security, which I measured mainly by the
libraries popularity, age, how recent and how frequent are the commits in its main branch of its repository and the
crate's reputation. After considering all these factors, `actix-web` seemed like the best choice, right next to
the `hyper` library.

To allow the user to customize the expected threshold level and the public key location, I added several command-line
parameters to the `server` crate. I used the `clap` crate for parsing the arguments, just like I did with the shares
generator crate for the same reasons I did earlier with the added benefit of consistency and not introducing another
dependency, especially if it is for the same purpose.

First, I implemented the route for retrieving the public key. To open the file and return it as a response, I used
the `NamedFile` interface from the `actix-files` crate. The reason is that the `actix-files` crate specializes in
hosting static files on the `actix-web` server, so it is safe to assume that it will be the most optimal implementation
for our purposes.

Lastly I was tasked with implementing the decryption scheme. I came up with a simple and performant design where server
receives and combines the key shares from all
the participants, decrypts the encrypted message and returns it as plaintext after the threshold of shares is received
for the message.

This concludes the implementation report. I'm happy with the outcome as it is simple, functional and efficient while
being certificate-less. Furthermore, I would like to add an abstraction for the encryption and decryption logic and the
secret sharing algorithm to allow for swapping those.
