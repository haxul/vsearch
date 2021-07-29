use std::fmt::Display;
use std::fs;
use std::io::Write;

pub mod storage;

pub struct FileStorage {
    file_path: String,
}

impl FileStorage {
    pub fn new(file_path: String) -> FileStorage {
        FileStorage { file_path }
    }
}

impl storage::Storage for FileStorage {
    fn save(&self, data_list: Vec<impl Display>) -> bool {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(&self.file_path)
            .unwrap();

        let strings: Vec<String> = data_list.iter().map(|v| format!("{}", v)).collect();
        for s in strings {
            let result = writeln!(file, "{}", s);
            if let Err(e) = result {
                println!("some gets wrong: {}", e);
                return false;
            };
        };
        true
    }

    fn get_storage_name(&self) -> &str {
        &self.file_path[..]
    }
}


