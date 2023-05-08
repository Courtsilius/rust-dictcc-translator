use regex::Regex;
use scraper::Html;

struct Language {
    value: String,
    name: String,
}

struct TranslationRequest {
    from: Language,
    to: Language,
    value: String,
}

fn main() {
    let input;
    let from;
    let to;
    (from, to, input) = get_input();

    if input.is_empty() {
        panic!("Cant translate without any input.");
    }

    let _german = Language {
        name: "Deutsch".to_string(),
        value: "de".to_string(),
    };

    let _english = Language {
        name: "Englisch".to_string(),
        value: "en".to_string(),
    };

    let from_language = Language {
        name: "".to_string(),
        value: from,
    };

    let to_language = Language {
        name: "".to_string(),
        value: to,
    };

    let result = get_translations(input, from_language, to_language);

    println!("{{\"values:\" {} }}", json::stringify(result));
}

fn get_input() -> (String, String, String) {
    let mut input: String = Default::default();
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 4 {
        panic!("Not enough inputs");
    }

    let from = args.get(1).unwrap();
    let to = args.get(2).unwrap();

    args.iter().skip(3).for_each(|x| {
        input.push(' ');
        input.push_str(x);
    });

    (from.to_string(), to.to_string(), input.trim().to_string())
}

// returns html data from a given url
fn scrape(url: &str) -> Html {
    let response = reqwest::blocking::get(url).unwrap().text().unwrap();

    scraper::Html::parse_document(&response)
}

fn filter(document: Html, left: &mut Vec<String>, right: &mut Vec<String>) {
    let title_selector = scraper::Selector::parse("td.td7nl").unwrap();

    let lines = document.select(&title_selector).map(|x| x.inner_html());
    let re =
        Regex::new(r"<[^>]*>|[^>]*</sup>|[0-9]*</div>|\{[^>]*\}|\[[^>]*\]|[^>]*</dfn>").unwrap();
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
        translation_request.from.value, translation_request.to.value, translation_request.value
    )
}

pub fn add(vec: &mut Vec<String>, word: String) {
    let is_present = vec.iter().any(|w| *(w) == word);

    if !is_present {
        vec.push(word);
    }
}

fn get_translations(input: String, from: Language, to: Language) -> Vec<String> {
    let words = process_translation_input(input);
    let translated = fetch_translations(generate_requests(words, from, to));

    let mut all: Vec<String> = vec![];
    for translation in translated {
        combine(translation.clone(), &mut all);
    }
    // sorting results by length
    all.sort_by_key(|b| std::cmp::Reverse(b.len()));
    all
}

fn process_translation_input(input: String) -> Vec<String> {
    let mut words: Vec<String> = vec![];
    for word in input.split(' ') {
        add(&mut words, word.to_string());
    }
    words
}

fn generate_requests(words: Vec<String>, from: Language, to: Language) -> Vec<TranslationRequest> {
    let mut translation_requests: Vec<TranslationRequest> = vec![];
    for needed_translation in words {
        let translation_request = TranslationRequest {
            from: Language {
                value: from.value.clone(),
                name: from.name.clone(),
            },
            to: Language {
                value: to.value.clone(),
                name: to.name.clone(),
            },
            value: needed_translation.to_string(),
        };
        translation_requests.push(translation_request)
    }
    translation_requests
}

fn combine(a: Vec<String>, c: &mut Vec<String>) {
    if c.is_empty() {
        *c = a;
    } else {
        let temp = c.clone();
        c.clear();
        for x in temp {
            for y in &a {
                c.push(format!("{} {}", x, y));
            }
        }
    }
}

fn fetch_translations(list: Vec<TranslationRequest>) -> Vec<Vec<String>> {
    let mut translations: Vec<Vec<String>> = vec![];

    for request in list {
        let translation_result: Vec<String> = translate(request);
        translations.push(translation_result);
    }
    translations
}

fn translate(translation_request: TranslationRequest) -> Vec<String> {
    let url = get_translation_url(translation_request);

    let document = scrape(&url);

    let mut left: Vec<String> = Vec::new();
    let mut right: Vec<String> = Vec::new();
    filter(document, &mut left, &mut right);

    // todo: add logic to decide which side needs to be returned based on layout of the table
    // and predefined name in given Language struct(s)
    right
}
