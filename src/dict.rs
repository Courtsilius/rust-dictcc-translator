pub mod dict_mod {
    use crate::helper::add;
    use crate::language::Language;
    use crate::translation_request::TranslationRequest;
    use regex::Regex;
    use scraper::Html;

    pub fn get_language(s: String) -> Language {
        match s.as_str() {
            "en" => Language::new("en".to_string(), "English".to_string()),
            "de" => Language::new("de".to_string(), "Deutsch".to_string()),
            _ => panic!("No valid language found."),
        }
    }

    pub fn translate(translation_request: TranslationRequest) -> Vec<String> {
        let url = get_translation_url(translation_request);

        let document = scrape(&url);

        let mut left: Vec<String> = Vec::new();
        let mut right: Vec<String> = Vec::new();
        filter(document, &mut left, &mut right);

        // todo: add logic to decide which side needs to be returned based on layout of the table
        // and predefined name in given Language struct(s)
        right
    }

    // returns html data from a given url
    fn scrape(url: &str) -> Html {
        let response = reqwest::blocking::get(url).unwrap().text().unwrap();

        scraper::Html::parse_document(&response)
    }

    fn filter(document: Html, left: &mut Vec<String>, right: &mut Vec<String>) {
        let title_selector = scraper::Selector::parse("td.td7nl").unwrap();

        let lines = document.select(&title_selector).map(|x| x.inner_html());
        let re = Regex::new(r"<[^>]*>|[^>]*</sup>|[0-9]*</div>|\{[^>]*\}|\[[^>]*\]|[^>]*</dfn>")
            .unwrap();
        let cleanup = Regex::new(r"[^>]*>|<[^>]*|&lt;[^>]*&gt;").unwrap();
        lines.zip(1..1000).for_each(|(item, number)| {
            // didn't wanna figure out why it doesn't work with one pass only so leaving
            // it like this for now
            let first_pass = re.replace_all(&item, "");
            let second_pass = re.replace_all(first_pass.trim(), "");
            let third_pass = cleanup.replace_all(second_pass.trim(), "");
            let trimmed: String = (third_pass.trim()).to_string();
            // translations in dict.cc in html are always from - to - from - to,
            // so adding every other word to their respective category
            if number % 2 == 0 {
                add(right, trimmed);
            } else {
                add(left, trimmed);
            }
        });
    }

    fn get_translation_url(translation_request: TranslationRequest) -> String {
        format!(
            "https://{}-{}.dict.cc/?s={}",
            translation_request.from().value(),
            translation_request.to().value(),
            translation_request.value()
        )
    }
}
