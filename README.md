# translation

[![Crates.io Version](https://img.shields.io/crates/v/translation.svg)](https://crates.io/crates/translation)
[![Docs.rs Version](https://docs.rs/translation/badge.svg)](https://docs.rs/translation)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square)](https://github.com/marek-g/rust-translation/blob/master/LICENSE-APACHE)
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](https://github.com/marek-g/rust-translation/blob/master/LICENSE-MIT)

Library for localization (text translation) of Rust applications.
 
## Features
 
- Autodetect user's locale (Windows, Unix).

- Small size - it doesn't depend on `regex` (like `locale_config` does).
 
- The localization files are embedded in the binary.
 
- It looks for localization files in the file system first, so new localization files can be added without application recompilation.
 
- It works great with `cargo-i18n`.

## Minimal example:

```rust
use translation::{tr_init, tr};

#[derive(rust_embed::RustEmbed)]
#[folder = "i18n/mo"]
struct Translations;

fn main() {
    tr_init!("locale", Translations);

    println!("{}", tr!("Hello, world!"));
}
```

The files from `i18n/mo` source folder are embedded in the executable file. The `tr_init!` macro looks for (in the following order):
- `locale/{lang}/LC_MESSAGES/{module_name}.mo` in the file system
- `locale/{lang}/{module_name}.mo` in the file system
- `{lang}/LC_MESSAGES/{module_name}.mo` in the embedded `Translations` struct
- `{lang}/{module_name}.mo` in the embedded `Translations` struct  

The `locale` folder is looked for relative to the application executable file.

For more examples how to use `tr!` macro (arguments, plurals, context etc.) please refer to [`tr`](https://crates.io/crates/tr) crate documentation.

## Dependencies

Current solution depends on the following crates:
- [`tr`](https://crates.io/crates/tr) - the `tr!` macro
- [`gettext`](https://crates.io/crates/gettext) - reimplementation of gettext in Rust
- [`rust_embed`](https://crates.io/crates/rust_embed) - to embed locale in executable file 

The crate was born because the `tr` crate (version 0.1.3 at the time of writing) is missing the `tr_init!` macro when used with `gettext` crate. Instead of creating pull request for `tr` crate I started experimenting with new crate and embedding translation files. In the future I plan to add support also for reading .po files directly (so no compilation step of .po files will be required).

## Usage

_Note. This instruction uses `cargo-i18` tool. Please also read https://github.com/kellpossible/cargo-i18n for more information._ 

### Configuration Steps

Install required tools:

```shell script
cargo install xtr
cargo install cargo-i18n
```

Add the following to your `Cargo.toml` dependencies:

```toml
translation = "1"
```

Create an `i18n.toml` file in the root directory of your crate:
 
 ```toml
fallback_language = "en"

[gettext]
target_languages = ["pl"]
output_dir = "i18n"
```
 
Run `cargo i18n` tool:

```shell script
cargo i18n
``` 

It scans the code and creates and updates the localization files. You can run it every time you want to update your localization files or compile `po` files.
