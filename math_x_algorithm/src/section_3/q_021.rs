use proconio::input;

pub(crate) fn main() {
    input! {
        n: i64,
        r: i64
    };

    let mut a = 1;
    let mut b = 1;
    for i in 1..=r {
        a *= i;
        b *= n - i + 1;
    }
    print!("{}", b / a);
}
