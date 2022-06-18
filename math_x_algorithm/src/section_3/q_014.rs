use proconio::input;

pub(crate) fn solution() {
    input! {
        n: i64,
    };
    let mut result: Vec<i64> = vec![];
    let mut current = n;
    let mut div = 2;
    while div * div <= current {
        if current % div == 0 {
            current /= div;
            result.push(div);
            div = 2;
        } else {
            div += 1;
        }
    }
    result.push(current);

    print!(
        "{}",
        result
            .into_iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
