use proconio::input;

pub(crate) fn solution() {
    input! {
        n: i64,
    };

    let mut current = 1;
    while current * current <= n {
        if n % current == 0 {
            println!("{}", current);
            println!("{}", n / current);
        }
        current += 1;
    }
}
