pub fn solve() -> bool {
    let s = "){";
    if s.len() % 2 == 1 {
        return false;
    }

    let mut stack: Vec<char> = Vec::new();

    for c in s.chars() {
        let is_closing = match c {
            ')' => true,
            ']' => true,
            '}' => true,
            _ => false,
        };

        if is_closing && Some(reverse(c)) != stack.pop() {
            return false;
        }
        if !is_closing {
            stack.push(c);
        }
    }
    if stack.is_empty() {
        return true;
    }
    false
}
fn reverse(c: char) -> char {
    match c {
        '}' => '{',
        ']' => '[',
        ')' => '(',
        _ => 'a',
    }
}
