pub fn solve() -> i32 {
    let nums = vec![1, 3, 5, 6];
    let target = 4;

    match nums.iter().position(|val| val == &target) {
        Some(idx) => {
            println!("num found");
            return idx as i32;
        }
        None => 0,
    };

    let new_pos = nums.iter().enumerate().find_map(|(i, val)| {
        if i == 0 {
            if &target < val {
                return Some(0);
            }
        }
        let curr = Some(val);
        let next = nums.get(i + 1);

        println!("curr: {curr:?}, next: {next:?}");
        let left = (curr.is_none() || curr.unwrap() <= &target);
        let right = (next.is_none() || next.unwrap() >= &target);

        println!("val: {val}, right: {right:?}, left: {left:?}");
        // (curr.is_none() || curr.unwrap() < val) && (next.is_none() || prev.unwrap() > val)
        if right && left {
            return Some(i + 1);
        }

        None
    });
    new_pos.unwrap() as i32
}
