use translation::{tr_embed, tr_init, tr};

tr_embed!("i18n/mo", Translations);

fn main() {
    tr_init!("locale", Translations);

    println!("{}", tr!("Hello, world!"));
}
