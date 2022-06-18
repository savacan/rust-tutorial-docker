pub(crate) fn solution(a: &Vec<i32>) -> i32 {
    let res: i32 = a.iter().sum();
    res % 100
}
