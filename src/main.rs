use std::fmt;

use clap::{Parser, ValueEnum};
pub mod rurl_get;

#[derive(Debug, Clone, ValueEnum)]
enum REQUEST_TYPE {
    GET,
    POST
}

impl fmt::Display for REQUEST_TYPE {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            REQUEST_TYPE::GET => write!(f, "GET"),
            REQUEST_TYPE::POST => write!(f, "POST"),
        }
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg()]
    url: String,

    #[arg(default_value_t = REQUEST_TYPE::GET)]
    req_type: REQUEST_TYPE, 

    #[arg(default_value_t = String::from("{}"))] //short, long, 
    d: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    println!("url {} , data {}", args.url, args.d);

    let a = args.url;

    let res = rurl_get::get::get(a).await;

    println!("response ---- {}", res);

}