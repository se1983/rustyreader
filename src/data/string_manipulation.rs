use std::cmp::Ordering;

#[allow(dead_code)]
pub fn truncate(word: String, length: usize) -> String {
    match word.chars().count().cmp(&length) {
        Ordering::Greater => format!("{:.wlen$} ...", word, wlen = (length - 4)),
        _ => word,
    }
}
