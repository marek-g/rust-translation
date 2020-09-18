#[cfg(target_family = "windows")]
mod windows;

#[cfg(not(target_os = "windows"))]
mod env;

pub struct Locale {
    pub language: String,
    pub region: Option<String>,
}

impl Locale {
    pub fn from(lang: &str) -> Option<Self> {
        let x: &[_] = &['.', ',', '@'];
        let lang_and_region: Vec<&str> = lang.split(x).next().unwrap().split('_').collect();
        let language = lang_and_region[0].to_owned();
        let region = if lang_and_region.len() > 1 {
            Some(lang_and_region[1].to_owned())
        } else {
            None
        };

        if language.is_empty() {
            None
        } else {
            Some(Locale {
                language,
                region,
            })
        }
    }
}
