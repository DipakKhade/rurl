use clap::{Parser, ValueEnum};

pub mod rurl_get;

#[derive(Debug, Clone, ValueEnum)]
enum RequestType {
    #[value(alias = "GET", alias = "Get")]
    get,

    #[value(alias = "POST", alias = "Post")]
    post,
}

#[derive(Parser, Debug, Clone)]
struct Args {
    #[arg(short = 'X', long = "method", value_enum, default_value_t = RequestType::get)]
    method: RequestType,

    #[arg(short, long)]
    url: String,

    #[arg(short, long, default_value_t = String::from("{}"))]
    data: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    dbg!(&args);

    match args.method {
        RequestType::get => {
            let result = rurl_get::get::get(args.url).await;   
            print!("{}", result);         
        },
        RequestType::post => {
            println!("method is POST--" )

        }
    }
}
