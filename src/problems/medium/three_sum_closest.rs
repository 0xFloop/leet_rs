use std::collections::{HashMap, HashSet};

pub fn solve() -> i32 {
    let target = 1;
    let nums = vec![0, 1, 2];

    let mut sorted_nums = nums;
    sorted_nums.sort_unstable();

    let mut curr_closest = i32::MAX;

    for i in 0..sorted_nums.len() - 2 {
        let num = sorted_nums[i];
        let mut small_idx = i + 1;
        let mut large_idx = sorted_nums.len() - 1;

        while small_idx < large_idx {
            let small = sorted_nums[small_idx];
            let large = sorted_nums[large_idx];

            let sum = num + small + large;

            if i32::abs_diff(sum, target) < i32::abs_diff(curr_closest, target) {
                curr_closest = sum;
            }

            if sum < target {
                small_idx += 1;
            } else if sum > target {
                large_idx -= 1;
            } else {
                while small_idx < large_idx && sorted_nums[small_idx] == small {
                    small_idx += 1;
                }
                while small_idx < large_idx && sorted_nums[large_idx] == large {
                    large_idx -= 1;
                }
            }
        }
    }

    curr_closest
}
