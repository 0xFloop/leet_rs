pub fn solve() -> Vec<i32> {
    let nums: Vec<i32> = vec![3, 3];
    let target: i32 = 6;

    for (index, num) in nums.iter().enumerate() {
        for (index2, num2) in nums.iter().enumerate() {
            if index2 == index {
                continue;
            }
            if num + num2 == target {
                return vec![index as i32, index2 as i32];
            }
        }
    }
    vec![0]
}
