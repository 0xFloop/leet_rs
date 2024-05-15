pub fn solve() -> String {
    let a = String::from("1010");
    let b = String::from("1011");

    let mut a = a.chars().rev();
    let mut b = b.chars().rev();

    let mut ret_str = String::new();
    let mut carry = false;
    loop {
        let mut a_curr = a.next();
        let mut b_curr = b.next();

        if a_curr.is_none() {
            if b_curr.is_some() {
                a_curr = Some('0');
            } else {
                break;
            }
        }
        if b_curr.is_none() && a_curr.is_some() {
            b_curr = Some('0');
        }

        if a_curr.unwrap() == '0' && b_curr.unwrap() == '0' {
            if carry {
                ret_str.push('1');
                carry = false;
            } else {
                ret_str.push('0');
            }

            continue;
        }
        if a_curr.unwrap() == '1' && b_curr.unwrap() == '1' {
            if carry {
                ret_str.push('1');
            } else {
                ret_str.push('0');
                carry = true;
            }
            continue;
        }
        if carry {
            ret_str.push('0');
        } else {
            ret_str.push('1');
        }
    }

    if carry {
        ret_str.push('1');
    }
    ret_str.chars().rev().collect::<String>()
}
