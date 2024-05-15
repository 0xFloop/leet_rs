use std::collections::HashMap;

//my solutions are very elementary compared to smarter (and faster) solutions I see online
pub fn solve() -> i32 {
    let string = String::from("bbtablud");
    let mut longest = 0;

    'loop1: for (i, c) in string.chars().enumerate() {
        let mut count = 1;
        let mut tmp_str = String::from(c);
        for c in string[i + 1..].chars() {
            println!("{tmp_str}");
            if tmp_str.contains(c) {
                if count > longest {
                    longest = count;
                }
                println!("loop contains letter, resetting for next letter");
                continue 'loop1;
            }

            count += 1;
            tmp_str.push_str(&c.to_string());
        }

        if count > longest {
            longest = count;
        }
    }
    longest
}
