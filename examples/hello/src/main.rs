use translation::{tr_init, tr};

#[derive(rust_embed::RustEmbed)]
#[folder = "i18n/mo"]
struct Translations;

fn main() {
    tr_init!("locale", Translations);

    println!("{}", tr!("Hello, world!"));
}
