pub fn add(vec: &mut Vec<String>, word: String) {
    let is_present = vec.iter().any(|w| *(w) == word);

    if !is_present {
        vec.push(word);
    }
}