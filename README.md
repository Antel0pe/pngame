Developed using: https://picklenerd.github.io/pngme_book/introduction.html

This project gives you a command line interface to hide secret messages in your PNGs without affecting how the image looks. PNGs are made up of a series of data chunks and each of those has a  chunk type. Chunk types contain metadata about the chunk like whether it is safe to copy, reserved or whether that chunk is private. We can take advantage of this and add our own data chunks containing secret messages and specify that these should not be displayed.

For more information: http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html#Chunk-naming-conventions

For our purposes here's what you need to know...
A chunk type is made up of 4 uppercase and lowercase ASCII letters.

The first letter should be lowercase.
The second letter should be lowercase.
The third letter should be uppercase.
The fourth letter can be either.

Example Chunk Type: ruSt

Build Instructions
```
cargo build -r
```

# Available Commands:
## To Encode a Secret Message in a PNG
```
pngame.exe encode <FILE PATH> <CHUNK TYPE> "<SECRET MESSAGE>"
pngame.exe encode png.png ruSt "this is my secret message"
```

## To Decode a Secret Message from a PNG
```
pngame.exe decode <FILE PATH> <CHUNK TYPE>
pngame.exe decode png.png ruSt
```

## To remove a chunk type from a PNG
```
pngame.exe remove <FILE PATH> <CHUNK TYPE>
pngame.exe remove png.png ruSt
```

## To print chunk types in a secret message
```
pngame.exe print <FILE PATH>
pngane.exe print png.png
```


