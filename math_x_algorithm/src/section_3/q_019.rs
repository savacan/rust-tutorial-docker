use std::collections::HashMap;

use proconio::input;

pub(crate) fn solution() {
    input! {
        n: usize,
        a: [i64; n]
    };

    let mut map = HashMap::<i64, i64>::new();
    for i in a.into_iter() {
        *map.entry(i).or_default() += 1;
    }

    print!(
        "{}",
        pick_two_pat(*map.get(&1).unwrap_or(&0))
            + pick_two_pat(*map.get(&2).unwrap_or(&0))
            + pick_two_pat(*map.get(&3).unwrap_or(&0))
    )
}

fn pick_two_pat(a: i64) -> i64 {
    if a < 2 {
        return 0;
    }
    a * (a - 1) / 2
}
