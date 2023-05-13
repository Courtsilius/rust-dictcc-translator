use clap::Parser;

use crate::dict::dict_mod::{get_language, translate};
use crate::helper::{add, combine};
use crate::language::Language;
use crate::translation::Translation;
use crate::translation_request::TranslationRequest;

mod dict;
mod helper;
mod language;
mod translation;
mod translation_request;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    from: String,

    #[arg(short, long)]
    to: String,

    #[clap(long, short, action)]
    input: String,
}

fn main() {
    let args = Args::parse();
    println!("F: {}, T: {}, I: {}", args.from, args.to, args.input);
    cont(args);
}

fn cont(args: Args) {
    let from_language = get_language(args.from);
    let to_language = get_language(args.to);

    let result = get_translations(args.input, from_language, to_language);

    let res_string = serde_json::to_string(&result).unwrap();
    println!("{}", res_string);
}

fn get_translations(input: String, from: Language, to: Language) -> Translation {
    let words = process_translation_input(input.clone());
    let translated = fetch_translations(generate_requests(words, from.clone(), to.clone()));

    let mut all: Vec<String> = vec![];
    for translation in translated {
        combine(translation.result().clone(), &mut all);
    }
    // sorting results by length
    all.sort_by_key(|b| std::cmp::Reverse(b.len()));
    Translation::new(TranslationRequest::new(input, from, to), all)
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

fn fetch_translations(list: Vec<TranslationRequest>) -> Vec<Translation> {
    let mut translations: Vec<Translation> = vec![];

    for request in list {
        translations.push(translate(request));
    }
    translations
}

pub fn process_translation_input(input: String) -> Vec<String> {
    let mut words: Vec<String> = vec![];
    for word in input.split(' ') {
        add(&mut words, word.to_string());
    }
    words
}
