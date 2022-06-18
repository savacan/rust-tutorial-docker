use std::collections::HashSet;

use proconio::input;

pub(crate) fn solution() {
  input!{
    n: i32,
    x: i32,
    y: i32,
  }

  let mut num_set = HashSet::<i32>::new();
  find_insert(&mut num_set, n, x);
  find_insert(&mut num_set, n, y);

  print!("{}", num_set.into_iter().count())
}

fn find_insert(num_set: &mut HashSet<i32>, n: i32, x: i32) {
  let mut current = x.clone();
  while current <= n {
    num_set.insert(current.clone());
    current += x;
  }
}