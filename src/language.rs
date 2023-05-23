use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Language {
    value: String,
    name: String,
    alt_name: String,
}
impl Language {
    pub fn value(&self) -> &String {
        &self.value
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn alt_name(&self) -> &String {
        &self.alt_name
    }

    pub fn new(v: String, n: String, a: String) -> Language {
        Language {
            value: v,
            name: n,
            alt_name: a,
        }
    }
}
