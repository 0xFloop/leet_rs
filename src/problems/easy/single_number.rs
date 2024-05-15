use std::{collections::HashMap, i32};

pub fn solve() -> i32 {
    let nums = vec![2, 2, 1];

    let mut cache: HashMap<i32, i32> = HashMap::new();
    for num in &nums {
        let count = cache.get(num);
        let _ = match count {
            Some(c) => cache.insert(*num, c + 1),
            None => cache.insert(*num, 1),
        };
    }
    for num in nums {
        if cache.get(&num).unwrap() < &2 {
            return num;
        }
    }
    0
}
