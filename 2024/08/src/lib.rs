use std::{fs::File, io::Write};

pub struct LogQuery<'a> {
    logs: &'a Vec<String>,
}

impl<'a> LogQuery<'a> {
    pub fn new(logs: &'a Vec<String>) -> Self {
        LogQuery { logs }
    }

    pub fn search(&self, keyword: &str) -> Vec<&'a String> {
        self.logs
            .iter()
            .filter(|log| log.contains(keyword))
            .collect()
    }

    pub fn export_to_file(&self, keyword: &str, path: &str) -> std::io::Result<()> {
        // 🎁 Your code here! 🎁
        let results = self.search(keyword);
        let mut file = File::create(path)?;
        for result in results {
            file.write_all(result.as_bytes())?;
            file.write_all(b"\n")?;
        }

        Ok(())
    }
}
