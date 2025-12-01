use std::{
    io::Read,
    path::{self, PathBuf},
};

#[derive(Default)]
pub(crate) struct ResourceManager {
    files: [path::PathBuf; 12],
}

impl ResourceManager {
    pub(crate) fn get_file(&self, key: usize) -> String {
        let path = self.files[key].to_path_buf();
        let mut file = std::fs::OpenOptions::new().read(true).open(path).unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        data
    }

    pub(crate) fn add_file(&mut self, key: usize, file_path: String) {
        let mut buf = PathBuf::new();
        buf.push(file_path);
        self.files[key] = buf;
    }
}
