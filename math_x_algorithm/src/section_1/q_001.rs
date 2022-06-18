pub(crate) struct FenwickTree {
    pub(super) tree: Vec<i32>,
}

impl FenwickTree {
    pub(crate) fn new(len: usize) -> Self {
        Self { tree: vec![0; len] }
    }

    pub(crate) fn add(&mut self, index: usize, value: i32) {
        let mut n = index + 1;
        while n <= self.tree.len() {
            self.tree[n - 1] += value;
            n += n & n.wrapping_neg()
        }
    }

    pub(crate) fn sum(&self, index: usize) -> i32 {
        let mut sum = 0;
        let mut n = index + 1;
        while n > 0 {
            sum += self.tree[n - 1];
            n -= n & n.wrapping_neg()
        }
        sum
    }
}
