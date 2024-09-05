pub fn solve() -> i32 {
    let dividend = 2147483647_i32;
    let divisor = 2_i32;

    // let dividend = 1_i32;
    // let divisor = -1_i32;

    let is_negative = (dividend.is_negative()) ^ (divisor.is_negative());

    let mut n = if dividend.is_negative() {
        dividend
    } else {
        -dividend
    };

    let d = if divisor.is_negative() {
        divisor
    } else {
        -divisor
    };

    let mut q = 0;

    while n <= d {
        q -= 1;
        n += d.abs();
    }
    if is_negative {
        q
    } else if q == i32::MIN {
        i32::MAX
    } else {
        -q
    }
}
