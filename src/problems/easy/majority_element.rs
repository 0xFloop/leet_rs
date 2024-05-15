use std::collections::HashMap;

pub fn solve() -> i32 {
    let nums = vec![1];
    // my naive solution first
    //
    // let mut cache: HashMap<i32, i32> = HashMap::new();
    // let maj = (nums.len() / 2) as i32;
    // println!("maj: {maj}");
    //
    // if nums.len() == 1 {
    //     return *nums.first().unwrap();
    // }
    // for num in nums {
    //     let count = cache.get(&num);
    //     let _ = match count {
    //         Some(cnt) => {
    //             if cnt + 1 > maj {
    //                 return num;
    //             }
    //             cache.insert(num, cnt + 1)
    //         }
    //         None => cache.insert(num, 1),
    //     };
    // }
    //
    //better solution
    let mut curr = nums[0];
    let mut count = 0;

    for num in nums.iter() {
        if *num == curr {
            count += 1;
        } else {
            count -= 1;
            if count < 0 {
                count = 0;
                curr = *num;
            }
        }
    }
    curr
}
