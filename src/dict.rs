pub mod dict_mod {
    use crate::helper::add;
    use crate::language::Language;
    use crate::translation::Translation;
    use crate::translation_request::TranslationRequest;
    use regex::Regex;
    use scraper::Html;

    pub fn get_language(s: String) -> Language {
        match s.as_str() {
            "en" => Language::new(
                "en".to_string(),
                "Englisch".to_string(),
                "English".to_string(),
            ),
            "de" => Language::new(
                "de".to_string(),
                "Deutsch".to_string(),
                "German".to_string(),
            ),
            // TODO: figure out a way to catch this panic
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

    fn get_first_appearance_in_html(html: &Html, selector: &str, pattern: &String) -> i32 {
        let language_selector = scraper::Selector::parse(selector).unwrap();
        let mut all_text: String = "".to_owned();
        let html_lines = html.select(&language_selector).map(|x| x.inner_html());
        html_lines.zip(1..1000).for_each(|(item, _number)| {
            all_text.push_str(&item);
        });
        let option = all_text.find(pattern);
        match option {
            None => 0,
            Some(usize) => usize.try_into().unwrap(),
        }
    }

    fn get_first_appearance_of_two_choices(
        html: &Html,
        selector: &str,
        pattern_one: &String,
        pattern_two: &String,
    ) -> i32 {
        let mut index = get_first_appearance_in_html(html, selector, pattern_one);
        if index == 0 {
            index = get_first_appearance_in_html(html, selector, pattern_two);
        }
        index
    }

    fn get_last_word(html: &Html) -> i32 {
        let selector = scraper::Selector::parse("table").unwrap();
        let mut document_string: String = "".to_owned();
        let document_map = html.select(&selector).map(|x| x.inner_html());
        document_map.zip(1..1000).for_each(|(item, _number)| {
            document_string.push_str(&item);
        });

        let mut index = reg_match(document_string);
        // if no other words are found we don't want to return too many translations, 6 seems like an OK amount
        if index == 0 {
            index = 6
        }
        index
    }

    fn reg_match(lang_lines: String) -> i32 {
        let mut index = 0;
        let mut regex = vec![
            r"Andere.*tr([1-9][0-9][0-9]|[1-9][0-9]|[0-9])",
            r"Others.*tr([1-9][0-9][0-9]|[1-9][0-9]|[0-9])", // TODO: more splits
        ];
        while index == 0 && !regex.is_empty() {
            let re = Regex::new(regex[0]).unwrap();
            let cap = re.captures(&lang_lines);
            regex.remove(0);
            index = match cap {
                None => 0,
                _ => cap
                    .unwrap()
                    .get(1)
                    .map_or("", |m| m.as_str())
                    .parse::<i32>()
                    .unwrap_or(0),
            }
        }
        index
    }

    fn filter(document: Html, translation_request: TranslationRequest) -> Translation {
        let language_selector = scraper::Selector::parse("td.td2").unwrap();
        let mut lang_lines: String = "".to_owned();
        let lines_lang = document.select(&language_selector).map(|x| x.inner_html());
        lines_lang.zip(1..1000).for_each(|(item, _number)| {
            lang_lines.push_str(&item);
        });

        let from_index = get_first_appearance_of_two_choices(
            &document,
            "td.td2",
            translation_request.from().name(),
            translation_request.from().alt_name(),
        );

        let to_index = get_first_appearance_of_two_choices(
            &document,
            "td.td2",
            translation_request.to().name(),
            translation_request.to().alt_name(),
        );

        let mut from_is_first: bool = true;
        if to_index < from_index {
            from_is_first = false;
        }

        let title_selector = scraper::Selector::parse("td.td7nl").unwrap();

        let max_index = get_last_word(&document) * 2;
        let re = Regex::new(r"<[^>]*>|[^>]*</sup>|[0-9]*</div>|\{[^>]*\}|\[[^>]*\]|[^>]*</dfn>")
            .unwrap();
        let cleanup = Regex::new(r"[^>]*>|<[^>]*|&lt;[^>]*&gt;").unwrap();
        let document_lines = document.select(&title_selector).map(|x| x.inner_html());

        let mut left: Vec<String> = Vec::new();
        let mut right: Vec<String> = Vec::new();
        document_lines.zip(1..max_index).for_each(|(item, number)| {
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
