pub fn solve() -> Vec<String> {
    let digits = String::from("23");

    let mut nums: Vec<String> = Vec::new();
    let mut tmp = String::new();
    if digits.is_empty() {
        return vec![];
    }
    recurse(&digits, digits.len(), &mut nums, &mut tmp, 0);

    nums
}

fn recurse(
    digits: &String,
    num_of_digits: usize,
    nums: &mut Vec<String>,
    tmp: &mut String,
    idx: usize,
) {
    // println!("nums: {nums:?}, tmp: {tmp}, idx: {idx}");
    if idx == num_of_digits {
        nums.push(tmp.clone());
        return;
    }

    let digit = digits.chars().nth(idx).unwrap();

    for letter in get_letters(digit).chars() {
        tmp.push(letter);
        recurse(digits, num_of_digits, nums, tmp, idx + 1);
        tmp.pop();
    }
}
fn get_letters(digit: char) -> String {
    match digit {
        '2' => "abc".to_string(),
        '3' => "def".to_string(),
        '4' => "ghi".to_string(),
        '5' => "jkl".to_string(),
        '6' => "mno".to_string(),
        '7' => "pqrs".to_string(),
        '8' => "tuv".to_string(),
        '9' => "wxyz".to_string(),
        _ => panic!("invalid digit"),
    }
}
