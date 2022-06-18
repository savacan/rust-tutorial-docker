use proconio::input;

// n^2許容する
pub(crate) fn solution() {
    input! {
        n: i32,
    };

    let mut res: Vec<i32> = vec![];
    for j in 2..=n {
        let mut f = true;
        for k in 2..j {
            if j % k == 0 {
                f = false
            }
        }
        if f {
            res.push(j)
        }
    }

    println!(
        "{}",
        res.into_iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}

// n^2以下になることを利用
pub(crate) fn solution_2() {
    input! {
        n: i32,
    };

    let mut res: Vec<i32> = vec![];

    for i in 2..=n {
        if is_prime(&i) {
            res.push(i)
        }
    }

    println!(
        "{}",
        res.into_iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}

fn is_prime(num: &i32) -> bool {
    let num = num.clone();
    let mut current = 2;
    let mut result = true;
    while current * current <= num {
        if num % current == 0 {
            result = false;
            break;
        }
        current += 1;
    }
    result
}
