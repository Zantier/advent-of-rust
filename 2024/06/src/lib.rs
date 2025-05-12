use std::cmp::Ordering;

// Write a function that returns the reference to the longer string
// without any new allocations
pub fn longer_wish<'a>(s1: &'a str, s2: &'a str) -> Option<&'a str> {
    // Your code here
    let len1 = s1.trim().chars().count();
    let len2 = s2.trim().chars().count();
    match len1.cmp(&len2) {
        Ordering::Greater => Some(s1),
        Ordering::Less => Some(s2),
        Ordering::Equal => None,
    }
}

#[cfg(test)]
mod test {
    use crate::longer_wish;

    #[test]
    fn test_length() {
        assert_eq!(longer_wish("hi", "hi"), None);
        assert_eq!(longer_wish("hi", "hello"), Some("hello"));
        assert_eq!(longer_wish("hello", "hi"), Some("hello"));
    }

    #[test]
    fn test_unicode() {
        assert_eq!(longer_wish("ᔐ", "hi"), Some("hi"));
    }
}
