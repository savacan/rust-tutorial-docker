use proconio::input;

pub(crate) fn main() {
    input! {
        n: usize,
        a: [i64; n]
    };

    // 全探査する
    let mut result: i64 = 0;

    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                for l in k + 1..n {
                    for m in l + 1..n {
                        if a[i] + a[j] + a[k] + a[l] + a[m] == 1000 {
                            result += 1;
                        }
                    }
                }
            }
        }
    }

    print!("{}", result);
}
