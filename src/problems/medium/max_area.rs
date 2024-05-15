use std::cmp;

pub fn solve() -> i32 {
    let heights: Vec<i32> = vec![2, 3, 4, 5, 18, 17, 6];

    let mut curr_largest_vol = 0;

    let mut left = (heights.first().unwrap(), 0);
    let mut right = (heights.last().unwrap(), heights.len() - 1);

    for _ in 0..heights.len() {
        if left.1 == right.1 {
            break;
        }
        let side = cmp::min(left.0, right.0);

        let volume = side * (right.1 - left.1) as i32;

        if volume > curr_largest_vol {
            curr_largest_vol = volume;
        }
        if left.0 > right.0 {
            right = (heights.get(right.1 - 1).unwrap(), right.1 - 1);
        } else {
            left = (heights.get(left.1 + 1).unwrap(), left.1 + 1);
        }
    }
    curr_largest_vol
}
