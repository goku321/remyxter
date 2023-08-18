use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Your YouTube API key
    api_key: String,

    /// Your YouTube Username
    username: String
}

fn main() {
    let args = Args::parse();
    println!("{}, {}", args.api_key, args.username);
}