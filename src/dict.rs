pub mod dict_mod {
    use crate::helper::add;
    use crate::language::Language;
    use crate::translation_request::TranslationRequest;
    use regex::Regex;
    use scraper::Html;
    use crate::translation::Translation;

    pub fn get_language(s: String) -> Language {
        match s.as_str() {
            "en" => Language::new("en".to_string(), "Englisch".to_string()),
            "de" => Language::new("de".to_string(), "Deutsch".to_string()),
            _ => panic!("No valid language found."),
        }
    }

    pub fn translate(translation_request: TranslationRequest) -> Translation {
        let url = get_translation_url(&translation_request);

        let document = scrape(&url);

        filter(document, translation_request)
    }

    // returns html data from a given url
    fn scrape(url: &str) -> Html {
        let response = reqwest::blocking::get(url).unwrap().text().unwrap();

        scraper::Html::parse_document(&response)
    }

    fn filter(document: Html, translation_request: TranslationRequest) -> Translation {
        let language_selector = scraper::Selector::parse("td.td2").unwrap();
        let mut lang_lines: String = "".to_owned();
        let lines_lang = document.select(&language_selector).map(|x| x.inner_html());
        lines_lang.zip(1..1000).for_each(|(item, _number)| {
            lang_lines.push_str(&item);
        });

        let v_f: Vec<_> = lang_lines.match_indices(translation_request.from().name()).map(|(i, _)|i).collect();
        let v_t: Vec<_> = lang_lines.match_indices(translation_request.to().name()).map(|(i, _)|i).collect();

        let from_index = v_f.first().unwrap();
        let to_index = v_t.first().unwrap();

        let mut from_is_first: bool = true;
        if to_index < from_index {
            from_is_first = false;
        }

        let title_selector = scraper::Selector::parse("td.td7nl").unwrap();

        let lines = document.select(&title_selector).map(|x| x.inner_html());
        let re = Regex::new(r"<[^>]*>|[^>]*</sup>|[0-9]*</div>|\{[^>]*\}|\[[^>]*\]|[^>]*</dfn>")
            .unwrap();
        let cleanup = Regex::new(r"[^>]*>|<[^>]*|&lt;[^>]*&gt;").unwrap();


        let mut left: Vec<String> = Vec::new();
        let mut right: Vec<String> = Vec::new();
        lines.zip(1..1000).for_each(|(item, number)| {
            // didn't wanna figure out why it doesn't work with one pass only so leaving
            // it like this for now
            let first_pass = re.replace_all(&item, "");
            let second_pass = re.replace_all(first_pass.trim(), "");
            let third_pass = cleanup.replace_all(second_pass.trim(), "");
            let trimmed: String = (third_pass.trim()).to_string();

            if number % 2 == 0 {
                add(&mut right, trimmed);
            } else {
                add(&mut left, trimmed);
            }
        });

        match from_is_first {
            true => Translation::new(translation_request, left),
            false => Translation::new(translation_request, right),
        }
    }

    fn get_translation_url(translation_request: &TranslationRequest) -> String {
        format!(
            "https://{}-{}.dict.cc/?s={}",
            translation_request.from().value(),
            translation_request.to().value(),
            translation_request.value()
        )
    }
}
