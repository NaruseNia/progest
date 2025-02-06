use std::{env, path::Path};

use metafile::create_meta_from_file;

fn main() {
    println!("{:?}", env::current_dir().unwrap());
    let path = Path::new("./test_files/test.txt");
    let mut meta = create_meta_from_file(path);
    meta.add_tag(uuidv7::create());
    println!("{:?}", meta);
    println!("{:?}", meta.raw_path());
}
