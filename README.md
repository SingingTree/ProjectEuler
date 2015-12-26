Attempts and solutions to Project Euler solutions, typically in rust.

Solutions source files are encrypted using OpenSSL aes256, with the answer to the solution used as the password to generate keys. So they can be decrypted as follows:

`openssl aes-256-cbc -d -in <file_name>`

after which you'll be prompted for a password, at which point you can use the answer to that solution to decrypt.
