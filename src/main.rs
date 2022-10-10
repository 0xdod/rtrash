use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use uuid::Uuid;

struct TrasherConfig {
    trash_dir: String,
    trash_meta_dir: String,
    trash_files_dir: String,
}

struct Trasher {
    config: TrasherConfig,
}

struct TrashFileMeta {
    file_name: String,
    file_path: String,
    file_uuid: Uuid,
    file_deleted_at: u128,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("err: no arguments supplied");
        println!("Usage: {} <file>", args[0]);
        return;
    }

    let trasher = Trasher::new(TrasherConfig {
        trash_dir: String::from("/home/damilola/.rtrash"),
        trash_meta_dir: String::from("/home/damilola/.rtrash/meta"),
        trash_files_dir: String::from("/home/damilola/.rtrash/files"),
    });

    for i in args[1..].iter() {
        let p = Path::new(i);
        if p.exists() {
            trasher.trash(p);
        } else {
            println!("file does not exist");
        }
    }
}

// TODO: ensure current directory/parent cannot be removed

impl Trasher {
    fn new(cfg: TrasherConfig) -> Trasher {
        if !Path::new(cfg.trash_dir.as_str()).exists() {
            fs::create_dir(cfg.trash_dir.as_str()).expect("Could not create trash directory");
            fs::create_dir(cfg.trash_meta_dir.as_str())
                .expect("Could not create trash meta directory");
            fs::create_dir(cfg.trash_files_dir.as_str())
                .expect("Could not create trash files directory");
        }
        Trasher { config: cfg }
    }

    fn create_meta(&self, fpath: &Path) -> TrashFileMeta {
        // create metadata and save in yml file
        let meta = TrashFileMeta {
            file_name: String::from(fpath.file_name().unwrap().to_str().unwrap()),
            file_path: String::from(fpath.canonicalize().unwrap().to_str().unwrap()),
            file_uuid: Uuid::new_v4(),
            file_deleted_at: get_current_timestamp_milli(),
        };

        // println!("timestamp: {}", meta.file_deleted_at);
        // println!("absolute file path: {}", meta.file_path);
        // println!("file name: {}", meta.file_name);
        // println!("internal file id: {:?}", meta.file_uuid);

        // create file to store metadata
        let mut meta_file = File::create(
            Path::new(self.config.trash_meta_dir.as_str()).join(meta.file_uuid.to_string()),
        )
        .unwrap();

        meta_file
            .write_all(
                format!(
                    "[meta]\noriginal_path = {}\ntimestamp = {}\nfilename = {}\n",
                    fpath.canonicalize().unwrap().to_str().unwrap(),
                    meta.file_deleted_at,
                    meta.file_name
                )
                .as_bytes(),
            )
            .unwrap();
        meta
    }

    fn trash(&self, fpath: &Path) {
        // create metadata and save in yml file
        let meta = self.create_meta(fpath);

        // move file to trash directory
        if fpath.is_dir() {
            println!("err: cannot move a directory yet");
            return;
        } else {
            fs::copy(
                meta.file_path.as_str(),
                Path::new(self.config.trash_files_dir.as_str()).join(meta.file_name.as_str()),
            )
            .unwrap();
        }

        // delete original file
        fs::remove_file(meta.file_path).unwrap();
    }
}

fn get_current_timestamp_milli() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}
