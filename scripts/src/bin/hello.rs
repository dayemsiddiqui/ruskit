use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    name: Option<String>,
}

fn main() {
    let args = Args::parse();
    match args.name {
        Some(name) => println!("Hello, {}!", name),
        None => println!("Hello, World!"),
    }
} 