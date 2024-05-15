pub fn solve() -> f64 {
    let nums1 = vec![1i32, 3];
    let nums2 = vec![2i32];
    let mut sorted: Vec<i32> = Vec::new();

    let mut idx_1 = 0;
    let mut idx_2 = 0;

    loop {
        let val1 = match nums1.get(idx_1) {
            Some(val) => val,
            None => {
                nums2[idx_2..].iter().for_each(|val| sorted.push(*val));
                break;
            }
        };

        let val2 = match nums2.get(idx_2) {
            Some(val) => val,
            None => {
                nums1[idx_1..].iter().for_each(|val| sorted.push(*val));
                break;
            }
        };

        match val1.cmp(val2) {
            std::cmp::Ordering::Less => {
                sorted.push(*val1);
                idx_1 += 1;
            }
            std::cmp::Ordering::Greater => {
                sorted.push(*val2);
                idx_2 += 1;
            }
            std::cmp::Ordering::Equal => {
                sorted.push(*val1);
                idx_1 += 1;
                sorted.push(*val2);
                idx_2 += 1;
            }
        }
    }

    if sorted.len() % 2 == 1 {
        return *sorted.get((sorted.len()).div_ceil(2) - 1).unwrap() as f64;
    }

    (*sorted.get((sorted.len() / 2) - 1).unwrap() as f64
        + *sorted.get(sorted.len() / 2).unwrap() as f64)
        / 2f64
}
