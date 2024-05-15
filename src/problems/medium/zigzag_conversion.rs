pub fn solve() -> String {
    let s = String::from("AB");

    let num_rows = 1;

    if num_rows == 1 {
        return s;
    }

    let mut tracker: Vec<Vec<char>> = vec![vec![' '; s.len()]; num_rows];

    let mut curr_row = 0;
    let mut curr_col = 0;

    let mut going_down = true;

    for c in s.chars() {
        if going_down {
            if curr_row < num_rows - 1 {
                tracker[curr_row][curr_col] = c;
                curr_row += 1;
            } else {
                going_down = false;
                tracker[curr_row][curr_col] = c;
                curr_col += 1;
                curr_row -= 1;
            }
        } else if !going_down {
            if curr_row > 0 {
                tracker[curr_row][curr_col] = c;
                curr_row -= 1;
                curr_col += 1;
            } else {
                going_down = true;
                tracker[curr_row][curr_col] = c;
                curr_row += 1;
            }
        }
    }

    let mut ret = String::new();

    for row in tracker {
        for col in row {
            if col != ' ' {
                ret.push(col);
            }
        }
    }
    ret
}
