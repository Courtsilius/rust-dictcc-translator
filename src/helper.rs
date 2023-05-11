pub fn add(vec: &mut Vec<String>, word: String) {
    let is_present = vec.iter().any(|w| *(w) == word);

    if !is_present && word.len() > 0 {
        vec.push(word);
    }
}
