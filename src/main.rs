use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use uuid::Uuid;

// check if the trash directory exists, if not create it
const TRASH_DIR: &str = "/home/damilola/.rtrash";
const TRASH_META_DIR: &str = "/home/damilola/.rtrash/meta";
const TRASH_FILES_DIR: &str = "/home/damilola/.rtrash/files";

fn main() {
    init();

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("err: no arguments supplied");
        println!("Usage: {} <file>", args[0]);
        return;
    }

    for i in args[1..].iter() {
        let p = Path::new(i);
        if p.exists() {
            trash(p);
        } else {
            println!("file does not exist");
        }
    }
}

// TODO: ensure current directory/parent cannot be removed

fn trash(fpath: &Path) {
    // create metadata and save in yml file
    // metadata consists of: original path, filename, timestamp, internal file id
    let file_id = Uuid::new_v4().to_string();
    let filename = fpath.file_name().unwrap().to_str().unwrap();
    let abs_filepath = fpath.canonicalize().unwrap();
    let abs_filepath = abs_filepath.to_str().unwrap();
    let timestamp = get_current_timestamp_milli();
    println!("timestamp: {}", timestamp);
    println!("absolute file path: {}", abs_filepath);
    println!("filename: {}", filename);
    println!("internal file id: {}", file_id);
    // create file to store metadata
    // save metadata to file in appropriate format and dir
    let mut file = File::create(Path::new(TRASH_META_DIR).join(file_id)).unwrap();
    file.write_all(
        format!(
            "[meta]\noriginal_path: {}\ntimestamp: {}\nfilename: {}\n",
            fpath.canonicalize().unwrap().to_str().unwrap(),
            timestamp,
            filename
        )
        .as_bytes(),
    )
    .unwrap();
    // move file to trash directory
    fs::copy(abs_filepath, Path::new(TRASH_FILES_DIR).join(filename)).unwrap();
    // delete original file
    fs::remove_file(abs_filepath).unwrap();
}

fn get_current_timestamp_milli() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

fn init() {
    if !Path::new(TRASH_DIR).exists() {
        fs::create_dir(TRASH_DIR).expect("Could not create trash directory");
        fs::create_dir(TRASH_META_DIR).expect("Could not create trash meta directory");
        fs::create_dir(TRASH_FILES_DIR).expect("Could not create trash files directory");
    }
}
