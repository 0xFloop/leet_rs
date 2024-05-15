use std::collections::VecDeque;

pub fn solve() -> String {
    let s = String::from("bbb");
    let mut curr_longest: VecDeque<char> = VecDeque::new();

    palindrome_odd(&s, &mut curr_longest);
    println!("Curr longest between: {curr_longest:?}");
    palindrome_even(&s, &mut curr_longest);

    curr_longest.into_iter().collect::<String>()
}
fn palindrome_odd(s: &str, curr_longest: &mut VecDeque<char>) {
    let s_vec = s.chars().collect::<VecDeque<char>>();

    let mut tmp: VecDeque<char> = VecDeque::new();

    for (i, c) in s_vec.iter().enumerate() {
        tmp.push_front(*c);

        for j in 1..=s_vec.len() / 2 {
            let prev = match i {
                (1..) => s_vec.get(i - j),
                _ => None,
            };

            let next = s_vec.get(i + j);

            match (prev, next) {
                (Some(c), Some(c2)) if c == c2 => {
                    tmp.push_front(*prev.unwrap());
                    tmp.push_back(*next.unwrap());
                }
                _ => {
                    break;
                }
            }
        }
        println!("TMP: {tmp:?}");
        if tmp.len() > curr_longest.len() {
            *curr_longest = tmp.clone();
        }
        tmp.clear();
    }
}
fn palindrome_even(s: &str, curr_longest: &mut VecDeque<char>) {
    let s_vec = s.chars().collect::<VecDeque<char>>();

    let mut tmp: VecDeque<char> = VecDeque::new();

    let doubles = s_vec.iter().zip(s_vec.iter().skip(1));

    for (i, (v1, v2)) in doubles.enumerate() {
        if v1 != v2 {
            continue;
        }
        tmp.push_front(*v1);
        tmp.push_front(*v2);

        for j in 1..=s_vec.len() / 2 {
            let prev = match i {
                (1..) => s_vec.get(i - j),
                _ => None,
            };

            let next = s_vec.get(i + j + 1);

            match (prev, next) {
                (Some(c), Some(c2)) if c == c2 => {
                    tmp.push_front(*prev.unwrap());
                    tmp.push_back(*next.unwrap());
                }
                _ => {
                    break;
                }
            }
        }
        if tmp.len() > curr_longest.len() {
            *curr_longest = tmp.clone();
        }
        tmp.clear();
    }
}
