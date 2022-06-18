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
        map.get(&100).unwrap_or(&0) * map.get(&400).unwrap_or(&0)
            + map.get(&200).unwrap_or(&0) * map.get(&300).unwrap_or(&0)
    )
}
