use std::fs;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    #[arg(short, long)]
    path: String
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args.path);

    let dirs: Vec<String> = fs::read_dir(args.path.clone())
        .unwrap()
        .filter(|d| d.as_ref().unwrap().path().is_dir() )
        .map(|d| d.unwrap().path().to_str().unwrap().to_string())
        .collect();

    println!("{:?}", dirs);
}
