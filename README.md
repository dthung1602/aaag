# AAAG - Another ASCII Art Generator

A CLI tool converts images to ASCII character (with color!)

## Build

Make sure you have `rust` and `cargo` installed in your system

Build command:

```shell
cargo build --release
```

The output is at `target/release/aaag`

## Usage

```
aaag [OPTIONS] --image <IMAGE_PATH> --output <OUTPUT_PATH>

Options:
  -i, --image <IMAGE_PATH>    
  -o, --output <OUTPUT_PATH>  
      --width <W>             If width and heigh are present, aaag will scale the image to the desired size
      --height <H>            If only one of width or height is present, aaag will keep the original aspect ratio
                              Otherwise, aaag will try to fit the image into the opened terminal
  -c, --color                 Print with color (the terminal must support 8bit rgb)
  -h, --help                  Print help
  -V, --version               Print version
```