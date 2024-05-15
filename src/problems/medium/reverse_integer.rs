use std::collections::VecDeque;

pub fn solve() -> i32 {
    let input = -563847412;

    let mut rev = input.to_string().chars().rev().collect::<VecDeque<char>>();
    println!("{}", i32::MIN);

    let mut cmp_val = i32::MAX.to_string().chars().collect::<VecDeque<char>>();

    let mut is_negative = false;
    if rev.iter().last().unwrap() == &'-' {
        cmp_val = i32::MIN.to_string().chars().collect::<VecDeque<char>>();
        is_negative = true;
        rev.pop_back();
        rev.push_front('-');
    }
    if (!is_negative && rev.len() < 10) || (is_negative && rev.len() < 11) {
        return rev.iter().collect::<String>().parse().unwrap();
    }

    println!(
        "{:?}",
        rev.iter().collect::<String>().parse::<i32>().unwrap()
    );
    for (inp, cmp) in rev.iter().zip(cmp_val) {
        if inp == &'-' {
            continue;
        }
        let inp = inp.to_digit(10).unwrap();
        let cmp = cmp.to_digit(10).unwrap();

        match cmp.cmp(&inp) {
            std::cmp::Ordering::Less => {
                println!("inp is greater than than cmp");
                return 0;
            }
            std::cmp::Ordering::Equal => {
                println!("inp is equal to cmp");
                continue;
            }
            std::cmp::Ordering::Greater => {
                println!("inp is less than cmp");
                break;
            }
        }
    }
    println!("{rev:?}");
    rev.iter().collect::<String>().parse().unwrap()
}
