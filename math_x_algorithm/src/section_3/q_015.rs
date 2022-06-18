use proconio::input;

pub(crate) fn solution() {
    input! {
        a: i64,
        b: i64,
    };

    let result = if a >= b { u_div(b, a) } else { u_div(a, b) };
    print!("{}", result);
}

fn u_div(s: i64, l: i64) -> i64 {
    if s == 0 {
        return l;
    }
    let mo = l % s;
    if s >= mo {
        u_div(mo, s)
    } else {
        u_div(s, mo)
    }
}
