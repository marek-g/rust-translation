use crate::locale::Locale;
use std::env;

impl Locale {
    pub fn current() -> Option<Self> {
        // LC_ALL overrides everything
        if let Ok(lang) = env::var("LC_ALL") {
            return Locale::from(&lang);
        }

        // LANG is default
        if let Ok(lang) = env::var("LANG") {
            return Locale::from(&lang);
        };

        None
    }
}
