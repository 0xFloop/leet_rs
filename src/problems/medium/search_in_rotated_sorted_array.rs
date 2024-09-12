pub fn solve() -> i32 {
    let nums = vec![5, 1, 3];
    let target = 3;

    let mut lo = 0;
    let mut hi = nums.len() - 1;

    while lo <= hi {
        let mid = (lo + hi) / 2;

        let l = nums[lo];
        let r = nums[hi];
        let m = nums[mid];

        if m == target {
            return mid as i32;
        }

        if l <= m {
            if l <= target && m >= target {
                hi = mid - 1;
            } else {
                lo = mid + 1;
            }
        } else {
            if m <= target && r >= target {
                lo = mid + 1;
            } else {
                hi = mid - 1;
            }
        }
    }
    -1
}
