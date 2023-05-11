use serde::{Deserialize, Serialize};

use crate::translation_request::TranslationRequest;

#[derive(Serialize, Deserialize)]
pub struct Translation {
    request: TranslationRequest,
    result: Vec<String>,
}
impl Translation {
    pub fn result(&self) -> &Vec<String> {
        &self.result
    }

    pub fn new(request: TranslationRequest, result: Vec<String>) -> Translation {
        Translation { request, result }
    }
}
