pub fn solve() -> bool {
    let nums: Vec<i32> = vec![1, 0, 1, 1];

    let k: i32 = 3;

    use std::collections::HashMap;
    let mut cache: HashMap<i32, usize> = HashMap::new();
    for (j, num) in nums.into_iter().enumerate() {
        let cache_data = cache.get(&num);
        match cache_data {
            Some(i) => {
                println!("num: {num}, initial: {i}, j: {j}, k: {k}");

                if j <= i + k as usize {
                    return true;
                }
                cache.insert(num, j);
            }
            None => {
                cache.insert(num, j);
            }
        }
    }
    false
}
