// Write a function that returns the reference to the longer string
// without any new allocations
pub fn longer_wish<'a>(s1: &'a str, s2: &'a str) -> Option<&'a str> {
    // Your code here
    let len1 = s1.trim().chars().count();
    let len2 = s2.trim().chars().count();
    if len1 > len2 {
        Some(s1)
    } else if len2 > len1 {
        Some(s2)
    } else {
        None
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
