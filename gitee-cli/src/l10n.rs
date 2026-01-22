pub enum Language {
    En,
    Zh,
}

impl Language {
    pub fn from_env() -> Self {
        if let Ok(lang) = std::env::var("LANG") {
            if lang.to_lowercase().contains("zh") {
                return Language::Zh;
            }
        }
        Language::En
    }
}

#[allow(dead_code)]
pub struct L10n {
    lang: Language,
}

impl L10n {
    pub fn new(lang_opt: Option<String>) -> Self {
        let lang = match lang_opt {
            Some(l) if l.to_lowercase() == "zh" => Language::Zh,
            Some(l) if l.to_lowercase() == "en" => Language::En,
            _ => Language::from_env(),
        };
        Self { lang }
    }

    #[allow(dead_code)]
    pub fn translate(&self, en: &str, zh: &str) -> String {
        match self.lang {
            Language::En => en.to_string(),
            Language::Zh => zh.to_string(),
        }
    }
}
