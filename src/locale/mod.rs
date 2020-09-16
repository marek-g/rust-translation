mod unix;

pub struct Locale {
    pub language: String,
    pub region: Option<String>,
}

impl Locale {
    pub fn from(lang: &str) -> Option<Self> {
        let lang = lang.as_bytes();
        if lang.len() >= 5 {
            if lang[2] == '_' as u8 {
                Some(Locale {
                    language: String::from_utf8_lossy(&lang[0..2]).into_owned(),
                    region: Some(String::from_utf8_lossy(&lang[3..5]).into_owned()),
                })
            } else {
                Some(Locale {
                    language: String::from_utf8_lossy(&lang[0..2]).into_owned(),
                    region: None,
                })
            }
        } else if lang.len() == 2 {
            Some(Locale {
                language: String::from_utf8_lossy(&lang[0..2]).into_owned(),
                region: None,
            })
        } else {
            None
        }
    }
}
