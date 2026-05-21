## Key art
Generate art that encodes arbitrary data.
### Motivation
OpenSSL provides a tool called random-art, which generates an image *derrived* from your public key. 
This image to key mapping is surjective, meaning two different keys could have the same random art. 
This tool prevents this issue by fully encoding public keys (or their fingerprints) inside of the image, while keeping the encoding in a way that looks similar to the original random-art.
Images can also be decoded to reveal the keys, meaning that instead of sharing your public key, you can also share this image. 
Think of it as a pretty qr code.
### Usage
This program provides a command line interface that is fairly easy to use. There are two subcommands for interfacing with the encoder and decoder.
The cli provides a global `--silent -s` flag which disables all standard output except if necessary.
#### Encode
Encode some bytes into an artwork
```
key_art encode [OPTIONS] <--file|--hex|--text> <input> <output>
```
Specify the input type with either 
- `--file, -f`, where `<input>` has to be a path
- `--hex -x`, where `<input>` has to be a hex string
- `--text -t` where `<input>` has to be a utf-8 string

If `<output>` is set, it must be a file path. The encoded artwork will the be written to said file. If `<output>` isn't set, the encoded artwork will be displayed via the standard output
#### Decode
```
key_art decode [OPTIONS] <input> <output>
```
If `<input>` is set, it must point to a file which contains an encoded artwork. If `<input>` isn't set, you will be asked to provide an artwork at runtime over the standard input.
The `<output>` parameter is similar to `encode`'s respective parameter. 
### Examples
```sh
$ ./key_art encode -x 24317827454D8A6AF33E526256E3555682464145 -s #My public key's fingerprint
╭──────────────╮
│ .....   -=...│
│        .*=  .│
│ -      = --  │
│  . .*= #.=-  │
│ .  .#=-= .. %│
│ .   += .+  - │
│   + -  .   +*│
╰──────────────╯
```
```sh
$ ./key_art encode -t 'Hello, World!' -s | ./key_art decode
As hex: 48656c6c6f2c20576f726c6421
As utf8: 'Hello, World!'
```

```sh
$ ./key_art encode -s -t 'I love geese!' ~/Pictures/love.txt
$ ./key_art decode -s ~/Pictures/love.txt ~/Documents/message.txt
$ cat ~/Documents/message.txt
I love geese!
```

### Misc
Try it out and decode these ^^
```
╭──────────────────╮
│ .  ---.+ - +. . .│
│#=-%% .. =- ..  - │
│#--+*  * * +.% .* │
│*= %+   . =.-+-.%=│
│    -     %.*=#* -│
│ . %-.  *- .-=-@+ │
│. .=- ..  .   ..- │
│.=   ...-.  =- - .│
│@.   .   ...=*  ..│
╰──────────────────╯
```
```
╭──────────────╮
│ .. ++--. =-  │
│ *    .     . │
│ --*++-    = -│
│.-  =.-  .  *-│
│.... ..  *  #.│
│   .... .-.-=*│
│   ... *-  .%#│
╰──────────────╯
```
```
╭────────────────────────────╮
│-         .==+-  ...   =+.--│
│  ..  .. .++== ... .=  @+= -│
│   .## .  %=-  .=   --=-.=.=│
│     = =+  . .=#**=+-.=.- -@│
│ --..@+=@ . ..@ *# -+#-*- - │
│  .*=+++     =.-@%* =---   .│
│*.%==++ #-=.   %#%## . =-  -│
│  -=+ == --%    ==#*=      .│
│%#===  .. - +   --.- . .   *│
│#+.=  -     ..  .   .#=.  + │
│*=.    =.+=. .=- @= # -   ..│
│*=-%+...- --..-=* - - =- .  │
│+#+ %+-@+-=-+% +  .. #+  =  │
│    +== -  == -=-*=*-=.#-== │
╰────────────────────────────╯
```
