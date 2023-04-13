# pngme
Hide secret messages in png files
It does this by getting all the png chunks of the given png image
and adding a new png chunk to the end after the IEND chunk


This project follow's picklenerds [pngme_book](https://picklenerd.github.io/pngme_book/introduction.html) 

## Usage

Clone this repository to your machine and use cargo install


## Commands
Supports encode, decode, remove, and print

```bash
pngme encode <FILE_PATH> <CHUNK_TYPE> <MESSAGE>
```
```bash
pngme encode ./test.png TemP "Secret Message"
```
```bash
pngme decode <FILE_PATH> <CHUNK_TYPE> 
```
```bash
pngme decode ./test.png TemP 
```
```bash
pngme remove <FILE_PATH> <CHUNK_TYPE> 
```
```bash
pngme remove ./test.png TemP 
```
```bash
pngme print <FILE_PATH> 
```
```bash
pngme print ./test.png 
```

