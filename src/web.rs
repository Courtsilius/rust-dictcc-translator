
use askama::Template;
use axum::{http::StatusCode, response::Html, response::IntoResponse, Form};

use crate::dict::dict_mod::get_language;
use crate::stupisaurus::stupisaurus_mod::stupi_translate;
use crate::translation::Translation;

#[derive(Template, Default)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    input: String,
    from_to: String,
}

#[derive(Template)]
#[template(path = "index_list.html")]
pub struct ListTemplate {
    input: String,
    from_to: String,
    from: String,
    to: String,
    translation: Translation,
}

pub async fn index() -> impl IntoResponse {
    let template = IndexTemplate {input: "".to_string(), from_to: "".to_string()};
    render_template(template)
}

pub async fn input(Form(form): Form<FormFields>,) -> impl IntoResponse {
    let mut lang_split = form.from_to.split("-");

    let a = lang_split.clone().filter_map(|x| x.parse::<String>().ok()).next();
    let c = lang_split.skip(1);
    let b = c.clone().filter_map(|x| x.parse::<String>().ok()).next();

    //let (from_language, to_language) = match lang_split {
    //    Some(_Vec) => (get_language(lang_split.iter().nth(0).unwrap_or("en").to_string()), get_language(lang_split.iter().nth(1).unwrap_or("de").to_string())),
    //    _ => (get_language("en".to_string()), get_language("de".to_string())),
    //};


    let result = stupi_translate(form.input.clone(), get_language(a.clone().unwrap()), get_language(b.clone().unwrap()), 50).await;

    let template = ListTemplate {input: form.input, translation: result, from_to: form.from_to, from: a.unwrap(), to: b.unwrap()};
    render_template(template)
}

#[derive(serde::Deserialize)]
pub struct FormFields {
    input: String,
    from_to: String,
}

fn render_template(template: impl Template) -> (StatusCode, Html<String>) {
    match template.render() {
        Ok(rendered) => {
            let code = StatusCode::OK;
            (code, Html(rendered))
        }
        Err(e) => {
            eprintln!("Failed to render template: {e:?}");

            (StatusCode::INTERNAL_SERVER_ERROR, Html(String::new()))
        }
    }
}
