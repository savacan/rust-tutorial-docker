fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true"); // 条件は真でした
    } else {
        println!("condition was false"); // 条件は偽でした
    }

    flow_tutorial();
    for_loop();
}

fn flow_tutorial() {
    let mut count = 3;

    while count > 0 {
        println!("count: {}", count);
        count -= 1;
    }
}

fn for_loop() {
    let arr = [10, 20, 30, 40, 50];
    let mut idx = 0;

    while idx < 5 {
        println!("value: {}", arr[idx]);
        idx += 1;
    }

    for element in arr.iter() {
        println!("value: {}", element);
    }

    for count in (1..4).rev() {
        println!("count down: {}", count);
    }
}
