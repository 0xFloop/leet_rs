pub fn solve() -> i32 {
    let nums = vec![4, 5, 6, 7, 0, 1, 2];
    let target = 3;
    //this does not solve the problem in the requested O(logn) time complexity
    match nums.iter().position(|&r| r == target) {
        Some(v) => return v as i32,
        None => return -1,
    }

    //this is my attempt to solve with the requestd complexity it using binary search but
    //the whole out of order thing is really confusing my brain
    //
    // let mut lo = 0;
    // let mut hi = nums.len() - 1;
    //
    // while lo <= hi {
    //     let mid = (lo + hi) / 2;
    //
    //     if nums[mid] == target {
    //         return mid as i32;
    //     }
    //
    //     if nums[lo] <= nums[mid] {
    //         if nums[lo] <= target && nums[hi] > target {
    //             hi = mid - 1;
    //         } else {
    //             lo = mid + 1;
    //         }
    //     } else {
    //     }
    // }
    // -1
}
