
use askama::Template;
use axum::{http::StatusCode, response::Html, response::IntoResponse, Form};

use crate::dict::dict_mod::get_language;
use crate::stupisaurus::stupisaurus_mod::stupi_translate;
use crate::translation::Translation;

#[derive(Template, Default)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    input: String,
}

#[derive(Template)]
#[template(path = "index_list.html")]
pub struct ListTemplate {
    input: String,
    translation: Translation,
}

pub async fn index() -> impl IntoResponse {
    let template = IndexTemplate {input: "".to_string()};
    render_template(template)
}

pub async fn input(Form(form): Form<FormFields>,) -> impl IntoResponse {
    let from_language = get_language("en".to_string());
    let to_language = get_language("de".to_string());


    let result = stupi_translate(form.input.clone(), from_language, to_language, 50).await;

    //let res_string = serde_json::to_string(&result).unwrap();
    //println!("{}", res_string);
    let template = ListTemplate {input: form.input, translation: result};
    render_template(template)
}

#[derive(serde::Deserialize)]
pub struct FormFields {
    input: String,
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
