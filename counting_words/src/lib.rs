use std::collections::HashMap;

pub fn counting_words(words: &str) -> HashMap<String, u32> {
    words
        .to_lowercase()
        .split(|c: char| !c.is_alphanumeric() && c != '\'')
        .map(|w| w.trim_matches('\''))
        .filter(|w| !w.is_empty())
        .fold(HashMap::new(), |mut map, w| {
            *map.entry(String::from(w)).or_default() += 1;
            map
        })
}
