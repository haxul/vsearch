use std::fmt::Display;
use std::fs;
use std::io::{Write, BufReader, BufRead};
use std::fs::File;
use std::path::Path;
use std::process::exit;
use regex::Regex;

pub mod storage;

pub struct FileStorage {
    file_path: String,
}

impl FileStorage {
    pub fn new(file_path: String) -> FileStorage {
        let exists = Path::new(&file_path).exists();
        if !exists {
            let _file = File::create(&file_path).unwrap();
        }
        FileStorage { file_path }
    }
}

impl storage::Storage for FileStorage {
    fn save(&self, row: impl Display) {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .append(true)
            .read(true)
            .open(&self.file_path)
            .unwrap();

        let buffer = BufReader::new(&file);
        let lines: Vec<String> = buffer
            .lines()
            .map(|line| { line.unwrap() })
            .collect();


        let re: Regex = regex::Regex::new(r"found:\s+\d+").unwrap();
        let check: String = re.replace_all(&row.to_string(), "").to_string();
        for line in lines {
            let line = re.replace_all(&line, "").to_string();
            if line.eq(&check) {
                return;
            }
        }

        let result = writeln!(file, "{}", row);
        if let Err(e) = result {
            println!("some gets wrong: {}", e);
            exit(1);
        };
    }

    fn get_storage_name(&self) -> &str {
        &self.file_path[..]
    }
}


