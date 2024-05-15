pub fn solve() -> i32 {
    let s = String::from("     fly me   to   the moon ");

    let words = s.trim().split(' ').collect::<Vec<&str>>();

    words.last().unwrap().len() as i32
}
