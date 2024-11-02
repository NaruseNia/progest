use std::{fs, path::PathBuf};

use indextree::{Arena, NodeId};
use serde::Serialize;
use serde_indextree::Node;

#[derive(Serialize, Debug)]
pub struct DirNode {
    path: String,
}

pub fn create_folder_structure(root: String) -> Vec<u8> {
    let mut arena: Arena<DirNode> = Arena::new();
    let root_node = arena.new_node(DirNode {
        path: String::from("root"),
    });

    travel(&root, &mut arena, root_node);

    let data = &Node::new(root_node, &arena);
    let out_data = bson::to_bson(&data).unwrap();

    bson::to_vec(&out_data).unwrap()
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
