pub struct LogQuery<'a> {
    logs: &'a Vec<String>,
}

// 1. Finish the implementation of LogQuery
impl<'a> LogQuery<'a> {
    // 2. Create a public associated function named `new()` that will take a reference to a vector of strings
    pub fn new(logs: &'a Vec<String>) -> Self {
        Self {
            logs,
        }
    }

    // 3. Create a public method named `search` that accepts a string slice and finds it from the logs and
    //    returns a vector of references to those logs.
    pub fn search(&self, text: &str) -> Vec<&str> {
        let mut res: Vec<&str> = Vec::new();
        for log in self.logs {
            if log.contains(text) {
                res.push(log);
            }
        }

        res
    }
}

#[cfg(test)]
mod test {
    use crate::LogQuery;

    #[test]
    fn test_search() {
        let logs = vec![
            "logs logs logs".to_string(),
            "01234".to_string(),
            "logs logs logs".to_string(),
            "logs 123 logs".to_string(),
        ];
        let logs = LogQuery::new(&logs);
        assert_eq!(logs.search("123"), vec!["01234", "logs 123 logs"]);
    }
}
