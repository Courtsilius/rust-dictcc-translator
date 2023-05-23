use askama::Template;
use axum::{http::StatusCode, response::Html, response::IntoResponse, Form};

use crate::dict::dict_mod::get_language;
use crate::helper::add;
use crate::language::Language;
use crate::stupisaurus::stupisaurus_mod::stupi_translate;
use crate::translation::Translation;
use crate::translation_request::TranslationRequest;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    input: String,
    from_to: String,
    from: String,
    to: String,
    translation: Translation,
}

#[derive(serde::Deserialize)]
pub struct FormFields {
    input: String,
    from_to: String,
}

pub async fn index() -> impl IntoResponse {
    let template = IndexTemplate {
        input: "".to_string(),
        from_to: "".to_string(),
        from: "".to_string(),
        to: "".to_string(),
        translation: Translation::new(
            TranslationRequest::new(
                "".to_string(),
                Language::new("".to_string(), "".to_string(), "".to_string()),
                Language::new("".to_string(), "".to_string(), "".to_string()),
            ),
            vec![],
        ),
    };
    render_template(template)
}

pub async fn input(Form(form): Form<FormFields>) -> impl IntoResponse {
    let mut languages: Vec<String> = vec![];
    for language in form.from_to.split('-') {
        add(&mut languages, language.to_string());
    }

    let from_language = get_language(languages.get(0).unwrap_or(&"en".to_string()).to_owned());
    let to_language = get_language(languages.get(1).unwrap_or(&"de".to_string()).to_owned());
    let result = stupi_translate(
        form.input.clone(),
        from_language.clone(),
        to_language.clone(),
        50,
    )
    .await;

    let template = IndexTemplate {
        input: form.input,
        translation: result,
        from_to: form.from_to,
        from: from_language.name().to_owned(),
        to: to_language.name().to_owned(),
    };
    render_template(template)
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
