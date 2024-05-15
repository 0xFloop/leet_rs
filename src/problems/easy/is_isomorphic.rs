use std::collections::HashMap;

pub fn solve() -> bool {
    let s = String::from("badc");
    let t = String::from("baba");

    let mut cache: HashMap<char, char> = HashMap::new();
    let mut cache2: HashMap<char, char> = HashMap::new();

    for (letter, t_letter) in s.chars().zip(t.chars()) {
        let cached = cache.get(&letter);
        let cached2 = cache2.get(&t_letter);

        match (cached, cached2) {
            (None, None) => {
                cache.insert(letter, t_letter);
                cache2.insert(t_letter, letter);
            }
            (Some(v1), Some(v2)) => {
                if v1 != &t_letter && v2 != &letter {
                    return false;
                }
            }
            _ => return false,
        }
    }
    true
}
