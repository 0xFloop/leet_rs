pub fn solve() -> i32 {
    let input = String::from("  0000000000012345678");
    let input = input.trim();

    let mut cached_num = String::new();
    let mut sign = 1;
    for (i, c) in input.chars().enumerate() {
        if i == 0 && c == '-' {
            sign = -1;
            continue;
        }
        if i == 0 && c == '+' {
            continue;
        }
        if c == '0' && cached_num.is_empty() {
            continue;
        }
        if c.is_ascii_digit() {
            cached_num.push(c);
        } else {
            break;
        }
    }

    println!("cached_num {cached_num:?}");
    if cached_num.is_empty() {
        return 0;
    }
    println!("sign: {sign}");
    println!("cached num: {cached_num:?}");
    println!(" num: {cached_num:?}");

    if cached_num.len() > 10 {
        if sign == -1 {
            return i32::MIN;
        }
        return i32::MAX;
    }

    let num = cached_num.parse::<u64>().unwrap();

    match sign {
        1 => {
            if num > i32::MAX as u64 {
                return i32::MAX;
            }
        }
        -1 => {
            if num > -(i32::MIN as i64) as u64 {
                return i32::MIN;
            }
        }
        _ => {}
    }
    num as i32 * sign
}
