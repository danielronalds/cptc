# Copy To Clipboard

`cptc` is a small helper CLI tool for quickly copying the 
contents of file/s to your systems clipboard.

## Installation

You can install `cptc` from cargo

```sh
$ cargo install cptc
```

## Usage

Using `cptc` is a simple as calling it with a file. For 
example to copy the contents of `test.txt` to your clipboard
you'd use the following.

```sh
$ cptc test.txt
```

### Merging multiple files

To copy multiple files, simply pass multiple files to `cptc`.
**Note:** The contents of the files are merged together with 
a newline inserted between them.

```sh 
$ cptc first.txt second.txt
```

### Wayland support

Currently on Wayland, the clipboard will be cleared when the 
program ends (A limitation of rust clipboard libraries). To 
overcome this use the `--pause` or `-p` flag to prevent the 
program closing before you've had a change to paste the 
contents of the file!
