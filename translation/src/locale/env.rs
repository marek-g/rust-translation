use crate::locale::Locale;
use std::env;

impl Locale {
    pub fn current() -> Option<Self> {
        if let Ok(lang) = env::var("LC_ALL") {
            if !lang.is_empty() {
                return Locale::from(&lang);
            }
        }

        if let Ok(lang) = env::var("LC_MESSAGES") {
            if !lang.is_empty() {
                return Locale::from(&lang);
            }
        }

        if let Ok(lang) = env::var("LANG") {
            if !lang.is_empty() {
                return Locale::from(&lang);
            }
        };

        None
    }
}
