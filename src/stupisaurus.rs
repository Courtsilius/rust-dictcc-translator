pub mod stupisaurus_mod {
    use crate::helper::{add, combine};
    use crate::language::Language;
    use crate::translation::Translation;
    use crate::translation_request::TranslationRequest;

    pub async fn stupi_translate(
        input: String,
        from: Language,
        to: Language,
        max: usize,
    ) -> Translation {
        let words = process_translation_input(input.clone());
        let translated =
            fetch_translations(generate_requests(words, from.clone(), to.clone())).await;

        let mut all: Vec<String> = vec![];
        for translation in translated {
            combine(translation.result().clone(), &mut all);
        }
        let max_items = if all.len() < max { all.len() } else { max };
        // sorting results by length
        all.sort_by_key(|b| std::cmp::Reverse(b.len()));
        Translation::new(
            TranslationRequest::new(input, from, to),
            all[0..max_items].to_vec(),
        )
    }

    fn generate_requests(
        words: Vec<String>,
        from: Language,
        to: Language,
    ) -> Vec<TranslationRequest> {
        let mut translation_requests: Vec<TranslationRequest> = vec![];
        for needed_translation in words {
            let translation_request =
                TranslationRequest::new(needed_translation.to_string(), from.clone(), to.clone());
            translation_requests.push(translation_request)
        }
        translation_requests
    }

    async fn fetch_translations(list: Vec<TranslationRequest>) -> Vec<Translation> {
        let mut tokio_spawns = Vec::new();
        for request in list {
            tokio_spawns.push(crate::dict::dict_mod::translate(request));
        }
        futures::future::join_all(tokio_spawns).await
    }

    pub fn process_translation_input(input: String) -> Vec<String> {
        let mut words: Vec<String> = vec![];
        for word in input.split(' ') {
            add(&mut words, word.to_string());
        }
        words
    }
}
