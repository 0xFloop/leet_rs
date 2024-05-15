use std::collections::HashSet;

pub fn solve() -> bool {
    let nums = vec![1, 2, 3, 4];

    let mut cache: HashSet<i32> = HashSet::new();
    for num in nums {
        if cache.contains(&num) {
            return true;
        }
        cache.insert(num);
    }
    false
}
