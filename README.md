## Key art
Generate art that encodes arbitrary data.
### Motivation
OpenSSL provides a tool called random-art, which generates an image *derrived* from your public key. 
This image to key mapping is surjective, meaning two different keys could have the same random art. 
This tool prevents this issue by fully encoding public keys (or their fingerprints) inside of the image, while keeping the encoding in a way that looks similar to the original random-art.
Images can also be decoded to reveal the keys, meaning that instead of sharing your public key, you can also share this image. 
Think of it as a pretty qr code.
### Usage
This program provides a command line interface that is fairly easy to use.
- Use `-e <Hex>` to encode a binary string
- Use `-d` to decode an image to get back its data
- Use `-t <Text>` to encode a unicode string
- Use `-s` to enable silent mode
- Use `--help` to display help
### Examples
```sh
$ ./key_art -e 24317827454D8A6AF33E526256E3555682464145 -s #My public key's fingerprint
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
$ ./key_art -t 'Hello, World!' -s | ./key_art -d
As hex: 48656c6c6f2c20576f726c6421
As utf8: 'Hello, World!'
```
### Misc
Try it out ^^
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
