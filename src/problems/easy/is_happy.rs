use std::collections::HashMap;

pub fn solve() -> bool {
    let n = 19i32;
    let mut n = n;

    let mut tmp;
    let mut cache: HashMap<i32, bool> = HashMap::new();

    loop {
        if n == 1 {
            return true;
        }
        if cache.insert(n, true).is_some() {
            return false;
        }
        tmp = 0;

        for c in n.to_string().chars() {
            let c_val = c.to_digit(10).unwrap() as i32;
            tmp += c_val * c_val;
        }

        n = tmp;
    }
}
