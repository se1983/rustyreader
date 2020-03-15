use std::cmp::Ordering;

#[allow(dead_code)]
pub fn truncate(word: String, length: usize) -> String {
    match word.chars().count().cmp(&length) {
        Ordering::Greater => format!("{:.wlen$} ...", word, wlen = (length - 4)),
        _ => word,
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_truncate_word() {
        let word = String::from("FooBar");
        assert_eq!(truncate(word, 5), "F ...")
    }

    #[test]
    fn test_dont_truncate_short_word() {
        let word = String::from("FooBar");
        assert_eq!(truncate(word, 6), "FooBar")
    }
}
