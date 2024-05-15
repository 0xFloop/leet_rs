pub fn solve() -> i32 {
    let s = "III";

    let mut val_vec: Vec<Option<i32>> = Vec::new();

    s.chars().for_each(|c| {
        let val = match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        };
        val_vec.push(Some(val));
    });
    let mut final_vals: Vec<i32> = Vec::new();

    let mut skip = false;
    for (i, val) in val_vec.iter().enumerate() {
        if skip {
            skip = false;
            continue;
        }
        if i == val_vec.len() - 1 {
            final_vals.push(val.unwrap());
            break;
        }

        let next_val = val_vec.get(i + 1).unwrap();

        if val < next_val {
            skip = true;
            final_vals.push(next_val.unwrap() - val.unwrap());
        } else {
            final_vals.push(val.unwrap());
        }
    }
    final_vals.iter().fold(0, |val, i| val + *i)
}
