use std::collections::HashMap;

use proconio::input;

pub(crate) fn main() {
    input! {
        n: usize,
        a: [i64; n]
    };

    let mut hmap = HashMap::<i64, i64>::new();
    for i in a {
        *hmap.entry(i).or_default() += 1;
    }

    let mut result = 0;

    for (k, v) in &hmap {
        let t = hmap.get(&(100000 - k));
        let res = match t {
            Some(ta) => {
                if k == &50000 {
                    ta * (ta - 1) / 2
                } else if k < &50000 {
                    ta * v
                } else {
                    0
                }
            }
            None => 0,
        };
        result += res;
    }

    print!("{}", result);
}
