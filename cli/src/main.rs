use clap::{Parser, Subcommand};
use std::{env, fs::File, io::Write, path::PathBuf};

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Create {
        root: String,
        #[arg(short, long)]
        output: Option<String>,
    },
}

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Create { root, output } => {
            if let Some(output) = output {
                env::set_current_dir(PathBuf::from(&output)).unwrap();
            } else {
                env::set_current_dir(PathBuf::from(&root)).unwrap();
            }

            let res = progetto::create_folder_structure(root);

            let mut file = File::create("struct.prg").unwrap();
            file.write_all(&res).unwrap();

            println!(
                "Create folder structure to: {:?}",
                env::current_dir().unwrap()
            );
        }
    }
}
