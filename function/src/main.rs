fn main() {
    another_function(five(), 4);
}

fn five() -> i32 {
    5
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is {}, y is {}", x, y); // 別の関数
}
