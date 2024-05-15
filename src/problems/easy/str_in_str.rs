pub fn solve() -> i32 {
    let haystack = String::from("badbutsad");
    let needle = String::from("sad");

    match haystack.find(&needle) {
        Some(pos) => return pos as i32,
        None => return -1,
    };
}
