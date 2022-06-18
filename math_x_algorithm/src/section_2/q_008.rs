use proconio::input;

pub(crate) fn solution() {
    input! {
      n: i32,
      s: i32,
    }

    // 脳筋スタイルでいく
    let mut cross_counter = 0;
    let mut counter = 0;
    let mut current_row = 1;

    while current_row <= n && current_row < s {
        for i in current_row..=n {
            if current_row + i > s {
                break;
            };
            if current_row == i {
                cross_counter += 1
            };
            counter += 1;
        }
        current_row += 1;
    }

    print!("{}", counter * 2 - cross_counter);
}
