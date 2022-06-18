use proconio::input;

pub(crate) fn solution() {
    input! {
        n: i64,
    };

    let mut current = 2;
    let mut result = true;
    while current * current <= n {
        if n % current == 0 {
            result = false;
            break;
        }
        current += 1;
    }
    print!("{}", if result { "Yes" } else { "No" });
}
