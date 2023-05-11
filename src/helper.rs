pub fn add(vec: &mut Vec<String>, word: String) {
    let is_present = vec.iter().any(|w| *(w) == word);

    if !is_present && !word.is_empty() {
        vec.push(word);
    }
}

pub fn combine(a: Vec<String>, c: &mut Vec<String>) {
    if c.is_empty() {
        *c = a;
    } else {
        let temp = c.clone();
        c.clear();
        for x in temp {
            for y in &a {
                c.push(format!("{} {}", x, y));
            }
        }
    }
}
