use clap::{Parser, Subcommand};
use indextree::{Arena, NodeId};
use serde::Serialize;
use serde_indextree::Node;
use std::fs;
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

#[derive(Serialize, Debug)]
struct DirNode {
    path: String,
}

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Create { root, output } => {
            let mut arena: Arena<DirNode> = Arena::new();
            let root_node = arena.new_node(DirNode {
                path: String::from("root"),
            });

            travel(&root, &mut arena, root_node);

            if let Some(output) = output {
                env::set_current_dir(PathBuf::from(&output)).unwrap();
            } else {
                env::set_current_dir(PathBuf::from(&root)).unwrap();
            }

            let data = &Node::new(root_node, &arena);
            let byted = bson::to_bson(&data).unwrap();

            let mut file = File::create("struct.prg").unwrap();
            file.write_all(bson::to_vec(&byted).unwrap().as_slice())
                .unwrap();

            println!(
                "Create folder structure to: {:?}",
                env::current_dir().unwrap()
            );
        }
    }
}

fn travel(path: &str, arena: &mut Arena<DirNode>, parent: NodeId) {
    let dirs = get_dirs(path);
    for entry in dirs {
        let dir_node = arena.new_node(DirNode {
            path: entry
                .file_name()
                .as_ref()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
        });
        parent.append(dir_node, arena);
        travel(entry.to_str().unwrap(), arena, dir_node);
    }
}

fn get_dirs(path: &str) -> Vec<PathBuf> {
    fs::read_dir(path)
        .unwrap()
        .filter(|entry| entry.as_ref().unwrap().path().is_dir())
        .map(|entry| entry.unwrap().path())
        .collect()
}
