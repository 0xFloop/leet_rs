pub fn solve() {
    let mut nums_1 = vec![4i32, 0, 0, 0, 0, 0];
    let m = 1;
    let mut nums_2 = vec![1i32, 2, 3, 5, 6];
    let n = 5;

    let nums1 = &mut nums_1;
    let nums2 = &mut nums_2;

    nums1.truncate(m);
    nums1.append(nums2);
    nums1.sort();
}
