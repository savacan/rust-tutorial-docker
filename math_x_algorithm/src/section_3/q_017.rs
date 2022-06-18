use proconio::input;

pub(crate) fn solution() {
    input! {
        n: usize,
        a: [i64; n]
    }

    let mut it = a.into_iter();
    let first = it.next().unwrap();
    let res = it.fold(first, |b, c| {
        let gcd = calc_gc_div(b, c);
        b / gcd * c
    });

    print!("{}", res);
}

fn calc_gc_div(a: i64, b: i64) -> i64 {
    let (s, l) = if a <= b { (a, b) } else { (b, a) };
    if s == 0 {
        return l;
    }
    calc_gc_div(s, l % s)
}
