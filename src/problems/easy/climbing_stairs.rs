use std::collections::HashMap;

pub fn solve() -> i32 {
    let n = 3;
    let mut memo: HashMap<i32, i32> = HashMap::new();
    climb(n, &mut memo)
}
fn climb(steps_left: i32, memo: &mut HashMap<i32, i32>) -> i32 {
    if steps_left <= 2 {
        return steps_left;
    }
    if let Some(res) = memo.get(&steps_left) {
        return *res;
    }

    let result = climb(steps_left - 1, memo) + climb(steps_left - 2, memo);
    memo.insert(steps_left, result);
    result
}
