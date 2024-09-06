pub fn solve() -> u32 {
    let s = "anagram";
    let t = "nagaram";

    let mut s_count: [i32; 26] = [0; 26];

    for letter in s.chars() {
        s_count[letter as usize - 97] += 1;
    }
    for letter in t.chars() {
        s_count[letter as usize - 97] -= 1;
    }
    for i in s_count {
        if i != 0 {
            println!("false");
            return 0;
        }
    }
    println!("true");
    1
}
