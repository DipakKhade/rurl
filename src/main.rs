use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    url: String,

    #[arg(short, long)]
    d: String,
}

fn main() {
    let args = Args::parse();

    println!("args {} , data {}", args.url, args.d);

}