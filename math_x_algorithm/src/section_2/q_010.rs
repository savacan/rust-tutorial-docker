use proconio::input;

pub(crate) fn solution() {
    input! {
        n: i64,
    }
    let mut result = 1;
    for i in 1..=n {
        result *= i;
    }

    print!("{}", result);
}
