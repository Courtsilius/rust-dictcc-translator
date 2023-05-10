#[derive(Clone)]
pub struct Language {
    value: String,
    name: String,
}
impl Language {
    pub fn value(&self) -> String {
        self.value
    }
    pub fn name(&self) -> String {
        self.name
    }

    pub fn new(v: String, n: String) -> Language {
        Language { value: v, name: n }
    }
}
