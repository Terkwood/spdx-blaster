# SPDX BLASTER!

🚧 This project is under construction! 🚧 

Put [SPDX license IDs](https://spdx.dev/ids/) in your (so far, rust) source files.

## Installation

```sh
git clone https://github.com/Terkwood/spdx-blaster
cd spdx-blaster
cargo install --path .
```

## Usage

For now, the tool can only deal with individual files (and not very many types of them).

```sh
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

