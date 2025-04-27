use std::collections::HashMap;

 pub fn counting_words(words: &str) -> HashMap<String, u32> {
    let mut word = HashMap::new();
    for w in words.split_whitespace() {
        let processed_word = w 
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '\'')
        .collect::<String>();

        if !processed_word.is_empty() {
            *word.entry(processed_word).or_insert(0) += 1;
        }
    }
    word
}

