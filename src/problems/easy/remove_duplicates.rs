use std::collections::HashSet;

pub fn solve() -> i32 {
    let nums: &mut Vec<i32> = &mut vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    let mut tracker: HashSet<i32> = HashSet::new();
    let mut res: Vec<i32> = Vec::new();
    for num in nums.iter() {
        if !tracker.contains(num) {
            res.push(*num);
            tracker.insert(*num);
        }
    }
    *nums = res;
    let count = &(nums.len() as i32);
    *count
}
