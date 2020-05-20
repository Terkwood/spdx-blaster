# SPDX BLASTER!

🚧 This project is under construction! 🚧 

Write [SPDX license IDs](https://spdx.dev/ids/) into your source files. 

## Installation

```sh
git clone https://github.com/Terkwood/spdx-blaster
cd spdx-blaster
cargo install --path .
```

## Usage

For now, the tool can only deal with individual files, although it knows about several different programming languages.

```sh
# an example rust file
spdx-blaster qux.rs
```

Having run the utility, you'll see a comment prepended at the top of the file:

```text
$ cat qux.rs

// SPDX-License-Identifier: MIT
extern crate clap;
extern crate env_logger;
extern crate log;
extern crate memmap;
```

### Specify a License

`spdx-blaster` doesn't know about very many licenses yet, but it
can handle some of the most popular ones.  Use the `--license` argument (or `-l`) to specify which license you want to use.

```sh
spdx-blaster -l gpl-3.0-or-later test.kt
spdx-blaster --license gpl-2.0-only test.java
```

You may omit several types of punctuation from the
license ID specification, but you must include
all the alphanumeric characters used in the license ID.

```sh
spdx-blaster -l apache20 test.cs
```

### Licenses Supported

The full list of licenses which are supported by `spdx-blaster`
can be [found in ids.rs](src/ids.rs).

## ⚠️ Warning!

We can't guarantee a crash-free experience in the case where another process is editing your source code at the same time as `spdx-blaster`. 

This is due to the use of `unsafe` memory-mapped files.

We plan to look into this further at some point in the future, and welcome suggestions for how we can make this utility both performant and safe. 
