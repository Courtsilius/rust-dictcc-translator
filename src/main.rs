use axum::{
    routing::{get, post},
    Router, Server,
};
use clap::Parser;

mod dict;
mod helper;
mod language;
mod stupisaurus;
mod translation;
mod translation_request;
mod web;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    from: String,

    #[arg(short, long)]
    to: String,

    #[clap(long, short, action)]
    input: String,

    #[arg(short, long, default_value_t = 100)]
    max: usize,
}

#[tokio::main]
async fn main() {
    println!("Starting webserver");
    let addr = "127.0.0.1:8123".parse().unwrap();

    let router = Router::new()
        .route("/", get(web::index))
        .route("/", post(web::input));

    Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
