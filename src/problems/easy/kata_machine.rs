pub fn solve() -> [i32; 10] {
    let mut arr: [i32; 10] = [3, 10, 44, 5, 32, 53, 2, 53, 5, 0];

    for i in 0..arr.len() {
        for j in 0..(arr.len() - 1 - i) {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
    arr
}
