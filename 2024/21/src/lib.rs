use std::{
    env::temp_dir,
    fs::{remove_file, File, OpenOptions},
    io::{Read, Seek, Write},
    path::PathBuf,
    sync::Mutex,
};

static GLOBAL_COUNTER: Mutex<u32> = Mutex::new(0);

pub struct TempFile {
    file_path: PathBuf,
    file: File,
}

impl TempFile {
    pub fn new() -> Result<Self, std::io::Error> {
        // Your code here...
        let mut counter = GLOBAL_COUNTER.lock().unwrap();
        let file_path = temp_dir().join(counter.to_string());
        *counter += 1;

        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .truncate(true)
            .create(true)
            .open(&file_path)?;

        Ok(Self {
            file_path,
            file,
        })
    }

    pub fn write(&mut self, data: &[u8]) -> Result<(), std::io::Error> {
        // Your code here...
        self.file.write_all(data)
    }

    pub fn read_to_string(&mut self) -> Result<String, std::io::Error> {
        // Your code here...
        self.file.rewind()?;
        let mut buf = String::new();
        self.file.read_to_string(&mut buf)?;
        Ok(buf)
    }

    pub fn path(&self) -> &PathBuf {
        &self.file_path
    }

    pub fn file(&self) -> &File {
        &self.file
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        _ = remove_file(&self.file_path);
    }
}
