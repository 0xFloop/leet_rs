pub fn solve() -> Vec<Vec<i32>> {
    let num_rows = 5i32;

    let num_rows = num_rows as usize;

    let mut final_vec: Vec<Vec<i32>> = vec![vec![1]];
    for i in 0..num_rows - 1 {
        let mut tmp_vec: Vec<i32> = vec![1];
        for j in 0..final_vec.len() {
            let new_val = final_vec.get(i).unwrap().get(j).unwrap()
                + final_vec.get(i).unwrap().get(j + 1).unwrap_or(&0);
            tmp_vec.push(new_val);
        }
        final_vec.push(tmp_vec);
    }
    final_vec
}
