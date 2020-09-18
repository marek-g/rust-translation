use crate::locale::Locale;
use std::env;

impl Locale {
    pub fn current() -> Option<Self> {
        if let Ok(lang) = env::var("LC_ALL") {
            return Locale::from(&lang);
        }

        if let Ok(lang) = env::var("LC_MESSAGES") {
            return Locale::from(&lang);
        }

        if let Ok(lang) = env::var("LANG") {
            return Locale::from(&lang);
        };

        None
    }
}
