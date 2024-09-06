pub fn solve() -> u32 {
    let nums = [0, 2, 3, 4, 6, 8, 9];
    if nums.is_empty() {
        return 0;
    }
    let mut prior_num = nums[0];
    let mut seq_start = nums[0];
    let mut ranges: Vec<String> = Vec::new();

    for num in nums {
        if num > prior_num + 1 {
            println!("break in nums here {:?} -> {:?}", num, prior_num);
            let mut new_range =
                String::from(seq_start.to_string() + " -> " + &prior_num.to_string());

            if prior_num == seq_start {
                new_range = String::from(seq_start.to_string());
            }
            seq_start = num;
            println!("new range: {:?}", new_range);

            ranges.push(new_range);
            //trigger a new sequence
        }

        prior_num = num;
    }

    if seq_start == prior_num {
        ranges.push(seq_start.to_string());
    } else {
        ranges.push(String::from(
            seq_start.to_string() + " -> " + &prior_num.to_string(),
        ));
    }
    println!("{:?}", ranges);
    100100
}
