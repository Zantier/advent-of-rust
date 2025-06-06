use std::collections::HashMap;

pub struct SantaList {
    // 1. Define the records field
    records: HashMap<String, bool>
}

impl SantaList {
    // 2. Implement the new method
    pub fn new() -> Self {
        Self { records: HashMap::new() }
    }

    // 3. Implement the add method
    pub fn add(&mut self, name: &str, behavior: bool) {
        self.records.insert(name.to_string(), behavior);
    }

    // 4. Implement the remove method
    pub fn remove(&mut self, name: &str) {
        self.records.remove(name);
    }

    // 5. Implement the get method
    pub fn get(&self, name: &str) -> Option<bool> {
        self.records.get(name).cloned()
    }

    // 6. Implement the count method
    pub fn count(&self) -> (usize, usize) {
        (
            self.list_by_behavior(true).len(),
            self.list_by_behavior(false).len(),
        )
    }

    // 7. Implement the list_by_behavior method
    pub fn list_by_behavior(&self, behavior: bool) -> Vec<String> {
        self.records.iter()
            .filter(|&(_name, &val)| val == behavior)
            .map(|(name, _val)| name.clone())
            .collect()
    }
}

impl Default for SantaList {
    fn default() -> Self {
        Self::new()
    }
}

pub fn main() {
    let mut santa_list = SantaList::new();

    santa_list.add("Alice", true);
    santa_list.add("Bob", false);
    santa_list.add("Charlie", true);

    if let Some(behavior) = santa_list.get("Alice") {
        println!(
            "Alice is on the {} list.",
            if behavior { "Nice" } else { "Naughty" }
        );
    }

    let (nice, naughty) = santa_list.count();
    println!(
        "Santa's list contains {} nice and {} naughty children.",
        nice, naughty
    );

    let nice_list = santa_list.list_by_behavior(true);
    println!("Nice children: {:?}", nice_list);

    let naughty_list = santa_list.list_by_behavior(false);
    println!("Naughty children: {:?}", naughty_list);

    santa_list.remove("Bob");
    let (nice, naughty) = santa_list.count();
    println!(
        "After removing Bob, Santa's list contains {} nice and {} naughty children.",
        nice, naughty
    );
}

#[cfg(test)]
mod test {
    use crate::SantaList;

    #[test]
    fn test_add_remove() {
        let mut list = SantaList::new();
        list.add("Alice", true);
        list.add("Bob", false);
        assert_eq!(list.get("Alice"), Some(true));
        assert_eq!(list.get("Bob"), Some(false));
        assert_eq!(list.get("Charlie"), None);

        list.remove("Bob");
        assert_eq!(list.get("Bob"), None);
    }
}
