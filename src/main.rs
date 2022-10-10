use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use uuid::Uuid;

struct TrashConfig {
    trash_dir: String,
    trash_meta_dir: String,
    trash_files_dir: String,
}

struct Trash {
    config: TrashConfig,
}

// struct MetaData {
//     file_name: String,
//     file_path: String,
//     file_uuid: String,
//     file_size: u64,
//     file_deleted_at: u64,
// }

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("err: no arguments supplied");
        println!("Usage: {} <file>", args[0]);
        return;
    }

    let cfg = TrashConfig {
        trash_dir: String::from("/home/damilola/.rtrash"),
        trash_meta_dir: String::from("/home/damilola/.rtrash/meta"),
        trash_files_dir: String::from("/home/damilola/.rtrash/files"),
    };
    let trash = Trash::new(cfg);

    for i in args[1..].iter() {
        let p = Path::new(i);
        if p.exists() {
            trash.trash(p);
        } else {
            println!("file does not exist");
        }
    }
}

// TODO: ensure current directory/parent cannot be removed

impl Trash {
    fn new(cfg: TrashConfig) -> Trash {
        if !Path::new(cfg.trash_dir.as_str()).exists() {
            fs::create_dir(cfg.trash_dir.as_str()).expect("Could not create trash directory");
            fs::create_dir(cfg.trash_meta_dir.as_str())
                .expect("Could not create trash meta directory");
            fs::create_dir(cfg.trash_files_dir.as_str())
                .expect("Could not create trash files directory");
        }
        Trash { config: cfg }
    }

    fn trash(&self, fpath: &Path) {
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
        let mut meta_file =
            File::create(Path::new(self.config.trash_meta_dir.as_str()).join(file_id)).unwrap();
        meta_file
            .write_all(
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
        fs::copy(
            abs_filepath,
            Path::new(self.config.trash_files_dir.as_str()).join(filename),
        )
        .unwrap();
        // delete original file
        fs::remove_file(abs_filepath).unwrap();
    }
}

fn get_current_timestamp_milli() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}
