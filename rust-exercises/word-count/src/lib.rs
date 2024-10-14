use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(phrase: &str) -> HashMap<String, u32> {
    let mut map = HashMap::new();
    for word in phrase
        .to_lowercase()
        .split_whitespace() {
        *map.entry(word.to_string()).or_insert(0) += 1;
    }
    map
}

//alternative with fold
/*use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(phrase: &str) -> HashMap<String, u32> {
    phrase
        .to_lowercase()
        .split_whitespace()
        .fold(HashMap::new(), |mut map, word| {
            *map.entry(word.to_string()).or_insert(0) += 1;
            map
        })
}*/