use clap::Parser;
pub mod rurl_get;

#[derive(Debug, Clone)]
enum REQUEST_TYPE {
    GET,
    POST
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg()]
    url: String,

    #[arg()]
    req_type: REQUEST_TYPE, 

    #[arg(short, long, default_value_t = String::from("{}"))]
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