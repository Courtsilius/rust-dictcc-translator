pub struct TranslationRequest {
    value: String,
    from: Language,
    to: Language,
}

impl TranslationRequest {
    pub fn value(&self) -> String {
        self.value
    }

    pub fn from(&self) -> Language {
        self.from
    }

    pub fn to(&self) -> Language {
        self.to
    }

    pub fn new(v: String, f: Language, t: Language) -> TranslationRequest {
        TranslationRequest { value: v, from: f, to: t}
    }
}