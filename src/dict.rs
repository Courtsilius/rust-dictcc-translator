pub mod dict_mod {
    use crate::helper::add;
    use crate::language::Language;
    use crate::translation::Translation;
    use crate::translation_request::TranslationRequest;
    use regex::Regex;
    use scraper::Html;

    pub fn get_language(s: String) -> Language {
        match s.as_str() {
            "en" => Language::new("en".to_string(), "Englisch".to_string(), "English".to_string()),
            "de" => Language::new("de".to_string(), "Deutsch".to_string(), "German".to_string()),
            _ => panic!("No valid language found."),
        }
    }

    pub async fn translate(translation_request: TranslationRequest) -> Translation {
        let url = get_translation_url(&translation_request);

        let document = scrape(url);

        filter(document.await, translation_request)
    }

    // returns html data from a given url
    async fn scrape(url: String) -> Html {
        let response = tokio::task::spawn_blocking(move || {
            reqwest::blocking::get(url).unwrap().text().unwrap()
        });
        let response = &response.await.unwrap();

        scraper::Html::parse_document(response)
    }

    fn string_index(source: &String, pattern: &String) -> Vec<usize> {
        source
            .match_indices(pattern)
            .map(|(i, _)| i)
            .collect()
    }

    fn filter(document: Html, translation_request: TranslationRequest) -> Translation {
        let language_selector = scraper::Selector::parse("td.td2").unwrap();
        let mut lang_lines: String = "".to_owned();
        let lines_lang = document.select(&language_selector).map(|x| x.inner_html());
        lines_lang.zip(1..1000).for_each(|(item, _number)| {
            lang_lines.push_str(&item);
        });

        let mut v_f = string_index(&lang_lines, translation_request.from().name());
        let mut from_index: &usize = v_f.first().unwrap_or(&0);

        let mut v_t = string_index(&lang_lines, translation_request.to().name());
        let mut to_index: &usize = v_t.first().unwrap_or(&0);


        if from_index == &0 {
            v_f = string_index(&lang_lines, translation_request.from().alt_name());
            from_index = v_f.first().unwrap_or(&0);
        }

        if to_index == &0 {
            v_t = string_index(&lang_lines, translation_request.to().alt_name());
            to_index = v_t.first().unwrap_or(&0);
        }

        let mut from_is_first: bool = true;
        if to_index < from_index {
            from_is_first = false;
        }

        let title_selector = scraper::Selector::parse("td.td7nl").unwrap();

        let mut lines = document.select(&title_selector).map(|x| x.inner_html());
        let mut all: String = "".to_owned();
        lines.zip(1..1000).for_each(|(item, _number)| {
            all.push_str(&item);
        });

        let v_max = string_index(&all, &"2".to_string());
        let max_index: &usize = v_max.first().unwrap_or(&0);


        if max_index == &0 {
            v_f = string_index(&all, &"Partial Matches".to_string());
            from_index = v_f.first().unwrap_or(&0);
        }


        let re = Regex::new(r"<[^>]*>|[^>]*</sup>|[0-9]*</div>|\{[^>]*\}|\[[^>]*\]|[^>]*</dfn>")
            .unwrap();
        let cleanup = Regex::new(r"[^>]*>|<[^>]*|&lt;[^>]*&gt;").unwrap();
        let mut lines_b = document.select(&title_selector).map(|x| x.inner_html());

        let mut left: Vec<String> = Vec::new();
        let mut right: Vec<String> = Vec::new();
        lines_b.zip(1..*max_index).for_each(|(item, number)| {
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

        Translation::new(
            translation_request,
            if from_is_first { right } else { left },
        )
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
