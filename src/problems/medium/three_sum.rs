use std::collections::{HashMap, HashSet};

pub fn solve() -> Vec<Vec<i32>> {
    let nums = vec![];

    //this is my naive approach, i kept trying to optimize it but ulitimately it is just too naive
    //for number sets with a significant number of unique values
    //
    //let mut set: HashSet<Vec<i32>> = HashSet::new();
    // let mut j_tracker: HashSet<(usize, usize)> = HashSet::new();
    // let mut i_tracker: HashSet<i32> = HashSet::new();
    //
    // let mut num_i;
    // let mut num_j;
    // let mut tracker;
    // let mut nums = nums;
    // let mut num_count: HashMap<i32, i32> = HashMap::new();
    //
    // let mut reduced_nums = Vec::new();
    //
    // for num in &nums {
    //     match num_count.get(num) {
    //         Some(val) => {
    //             if val < &3 {
    //                 num_count.insert(*num, val + 1);
    //                 reduced_nums.push(*num);
    //             }
    //         }
    //         None => {
    //             num_count.insert(*num, 1);
    //             reduced_nums.push(*num);
    //         }
    //     }
    // }
    //
    // reduced_nums.sort_unstable();
    //
    // for (val, count) in num_count {
    //     println!("val: {val}, count: {count}");
    //     println!();
    // }
    //
    // let len = reduced_nums.len();
    //
    // for i in 0..len {
    //     num_i = reduced_nums[i];
    //     if i_tracker.get(&num_i).is_some() {
    //         continue;
    //     };
    //     i_tracker.insert(num_i);
    //     tracker = num_i;
    //     for j in i + 1..len {
    //         if j_tracker.get(&(i, j)).is_some() {
    //             continue;
    //         }
    //         j_tracker.insert((i, j));
    //         num_j = reduced_nums[j];
    //         tracker += num_j;
    //         for num_k in reduced_nums.iter().take(len).skip(j + 1) {
    //             let tmp = vec![num_i, num_j, *num_k];
    //             if tracker + num_k == 0 {
    //                 if set.get(&tmp).is_some() {
    //                     continue;
    //                 }
    //
    //                 set.insert(vec![num_i, num_j, *num_k]);
    //             }
    //         }
    //         tracker -= num_j;
    //     }
    // }

    //here is a much more effecient algorithm i learned online for solving this type of problem

    let mut sorted_nums = nums;
    sorted_nums.sort_unstable();
    let mut set: HashSet<Vec<i32>> = HashSet::new();

    for i in 0..sorted_nums.len() - 2 {
        let num = sorted_nums[i];
        let mut small_idx = i + 1;
        let mut large_idx = sorted_nums.len() - 1;

        while small_idx < large_idx {
            let small = sorted_nums[small_idx];
            let large = sorted_nums[large_idx];

            let sum = num + small + large;
            if sum < 0 {
                small_idx += 1;
            } else if sum > 0 {
                large_idx -= 1;
            } else {
                set.insert(vec![num, small, large]);
                while small_idx < large_idx && sorted_nums[small_idx] == small {
                    small_idx += 1;
                }
                while small_idx < large_idx && sorted_nums[large_idx] == large {
                    large_idx -= 1;
                }
            }
        }
    }

    set.into_iter().collect::<Vec<Vec<i32>>>()
}
