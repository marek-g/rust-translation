pub use tr::tr;

use crate::locale::Locale;
use rust_embed::RustEmbed;
use std::path::PathBuf;

mod locale;

#[macro_export]
macro_rules! tr_embed {
    ($path:expr, $embedded:ident) => {
        #[derive(rust_embed::RustEmbed)]
        #[folder = $path]
        struct $embedded;
    }
}

#[macro_export]
macro_rules! tr_init {
    ($path:expr, $embedded:ty) => {
        $crate::internal::tr_init::<$embedded>(module_path!(), $path)
    };
}

pub mod internal {
    use rust_embed::RustEmbed;
    use crate::{locale, try_read_locale};

    pub fn tr_init<E: RustEmbed>(module: &'static str, local_folder: &str) {
        if let Some(locale) = locale::Locale::current() {
            if let Some(content) = try_read_locale::<E>(module, &locale, local_folder) {
                let catalog = gettext::Catalog::parse(&content[..]).expect("could not parse the catalog");
                tr::internal::set_translator(module, catalog);
            }
        }
    }
}

fn try_read_locale<E: RustEmbed>(module: &str, locale: &Locale, local_folder: &str) -> Option<Vec<u8>> {
    // try to read from file first
    if let Some(content) = try_read_from_file(module, locale, local_folder) {
        Some(content)
    } else {
        // if there is no file, try to read from embedded file
        try_read_from_embedded::<E>(module, locale)
    }
}

fn try_read_from_file(module: &str, locale: &Locale, local_folder: &str) -> Option<Vec<u8>> {
    let exe_path = std::env::current_exe().unwrap();
    let exe_folder = exe_path.parent().unwrap();
    let locale_paths = locale_to_paths(module, locale);
    for locale_path in locale_paths {
        let full_path = exe_folder.join(local_folder).join(locale_path);
        if let Some(content) = try_read_from_filepath(full_path) {
            return Some(content)
        }
    }
    None
}

fn try_read_from_embedded<E: RustEmbed>(module: &str, locale: &Locale) -> Option<Vec<u8>> {
    let locale_paths = locale_to_paths(module, locale);
    for locale_path in locale_paths {
        if let Some(content) = E::get(&locale_path.to_string_lossy().into_owned()) {
            return Some(content.to_vec())
        }
    }
    None
}

fn try_read_from_filepath(filepath: PathBuf) -> Option<Vec<u8>> {
    if let Ok(content) = std::fs::read(filepath) {
        Some(content)
    } else {
        None
    }
}

fn locale_to_paths(module: &str, locale: &Locale) -> Vec<PathBuf> {
    let mut possible_paths = Vec::new();
    if let Some(region) = &locale.region {
        possible_paths.push(PathBuf::from(&format!("{}_{}/{}.mo", locale.language, region, module)));
    }
    possible_paths.push(PathBuf::from(&format!("{}/{}.mo", locale.language, module)));
    possible_paths
}
