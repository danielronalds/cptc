# Copy To Clipboard

`cptc` is a small helper CLI tool for quickly copying the 
contents of file/s to your systems clipboard.

## Installation

You can install `cptc` from cargo

```sh
$ cargo install cptc
```

## Usage

```
usage: cptc [OPTIONS] [FILE]...

A cli tool for copying file/s contents to your clipboard

Options:
  --help       -h      Print this menu and exit
  --version    -v      Print the version number and exit
  --verbouse   -v      Print what is copied
  --pause      -p      Pauses exiting of the program until ENTER is pressed

```

### Wayland support

Currently on Wayland, the clipboard will be cleared when the 
program ends (A limitation of rust clipboard libraries). To 
overcome this use the `--pause` or `-p` flag to prevent the 
program closing before you've had a chance to paste the
contents of the file!

The program should auto-pause when it detects Wayland, however
if you can't seem to paste the content you've copied, this
might be the issue

