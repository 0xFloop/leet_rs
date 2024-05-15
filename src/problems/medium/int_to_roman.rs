pub fn solve() -> String {
    let num: i32 = 3749;
    let num = num.to_string();
    let mut tracker = Vec::new();

    println!("{num}");
    for (i, digit) in num.chars().rev().enumerate() {
        println!("i: {i}, digit: {digit}");
        match i {
            //ones digit
            0 => match digit {
                '1' => tracker.push("I"),
                '2' => tracker.push("II"),
                '3' => tracker.push("III"),
                '4' => tracker.push("IV"),
                '5' => tracker.push("V"),
                '6' => tracker.push("VI"),
                '7' => tracker.push("VII"),
                '8' => tracker.push("VIII"),
                '9' => tracker.push("IX"),
                '0' => {}
                _ => {
                    panic!("invalid ones digit")
                }
            },

            //tens digit
            1 => match digit {
                '1' => tracker.push("X"),
                '2' => tracker.push("XX"),
                '3' => tracker.push("XXX"),
                '4' => tracker.push("XL"),
                '5' => tracker.push("L"),
                '6' => tracker.push("LX"),
                '7' => tracker.push("LXX"),
                '8' => tracker.push("LXXX"),
                '9' => tracker.push("XC"),
                '0' => {}
                _ => {
                    panic!("invalid tens digit")
                }
            },

            //hundreds digit
            2 => match digit {
                '1' => tracker.push("C"),
                '2' => tracker.push("CC"),
                '3' => tracker.push("CCC"),
                '4' => tracker.push("CD"),
                '5' => tracker.push("D"),
                '6' => tracker.push("DC"),
                '7' => tracker.push("DCC"),
                '8' => tracker.push("DCCC"),
                '9' => tracker.push("CM"),
                '0' => {}
                _ => {
                    panic!("invalid hundreds digit")
                }
            },

            //thousands digit
            3 => {
                let digit = digit.to_digit(10).unwrap() as usize;
                for _ in 0..digit {
                    tracker.push("M");
                }
            }

            _ => {}
        }
    }
    println!("tracker: {tracker:?}");

    tracker
        .into_iter()
        .rev()
        .fold(String::new(), |acc, v| acc + v)
}
