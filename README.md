## PngCrypt

This application allows users to encrypt sensitive texts into png images. The encrypted texts are stored in the png image while still keepoing the image valid, anc can still be opened by image viewing applications.

*Compiler support: requires rustc 1.58*
___
### Details

 - Encoding: You can embed secret texts into images with the encode command.
    ```bash
    $ ./pngcrypt encode -f "neutral.png" -m "This is a highly classified information. Not to be transmited on public channels." --output-file "decoy.png"
    ```
	  You should get an output similar to
	  ```bash
    $ Secret encoded successfully The token is wbXH, please keep it a secret. It will be used for decoding your message.
    ```

- Decoding: You can decode secret your message from a png file with the decode command.
    ```bash
    $ ./pngcrypt decode -f "decoy.png" -c wbXH
    $ This is a highly classified information. Not to be transmited on public channels. 
    ```

- Cleaning: You can strip an image of secret messages with the remove command. The output will be the secret message.
    ```bash
    $ ./pngcrypt remove -f "decoy.png" -c wbXH
    $ This is a highly classified information. Not to be transmited on public channels. 
    ```
ROADMAP

 - [ ] Encrypt the embedded message with AES CTR and store the private key in another decoy png. Secret messages will now be decode by a suitable png image.
