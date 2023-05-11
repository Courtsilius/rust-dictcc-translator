use crate::dict::dict_mod::{get_language, translate};
use crate::helper::add;
use crate::language::Language;
use crate::translation_request::TranslationRequest;

mod dict;
mod helper;
mod language;
mod translation_request;

fn main() {
    let input;
    let from;
    let to;
    (from, to, input) = get_input();

    if input.is_empty() {
        panic!("Cant translate without any input.");
    }
    let from_language = get_language(from);
    let to_language = get_language(to);

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
        let translation_request =
            TranslationRequest::new(needed_translation.to_string(), from.clone(), to.clone());
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
