use clap::Parser;

pub mod rurl_get;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    url: String,

    #[arg(short, long)]
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