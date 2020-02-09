use std::cmp::Ordering;

pub fn truncate(word: String, length: usize) -> String {
    match word.chars().count().cmp(&length) {
        Ordering::Greater => {
            word.chars().collect::<Vec<_>>()[0..(length - 4)]
                .iter()
                .cloned()
                .collect::<String>()
                + "..."
        }
        _ => word,
    }
}
