use crate::dict::dict_mod::get_language;
use clap::Parser;

use crate::stupisaurus::stupisaurus_mod::stupi_translate;

mod dict;
mod helper;
mod language;
mod stupisaurus;
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

    #[arg(short, long, default_value_t = 100)]
    max: usize,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let from_language = get_language(args.from);
    let to_language = get_language(args.to);

    let result = stupi_translate(args.input, from_language, to_language, args.max).await;

    let res_string = serde_json::to_string(&result).unwrap();
    println!("{}", res_string);
}
