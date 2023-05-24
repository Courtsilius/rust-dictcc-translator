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
    #[arg(short, long, default_value_t = {"0.0.0.0:8000".to_string()})]
    socket: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    println!("Starting webserver");

    let addr = args.socket.parse().unwrap();

    let router = Router::new()
        .route("/", get(web::index))
        .route("/", post(web::input))
        .route("/beta", get(web::beta))
        .route("/beta", post(web::beta_input));
    Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
