use std::{iter::once, ops::AddAssign};

#[derive(Debug)]
pub struct FenwickTree<T: AddAssign + Default + Copy> {
    tree: Vec<T>,
}

// Tree is zero-indexed
impl<T: AddAssign + Default + Copy> FenwickTree<T> {
    pub fn new(v: Vec<T>) -> Self {
        let mut tree: Vec<T> = once(T::default()).chain(v).collect();

        for i in 1..tree.len() {
            let p = i + (i & (!i + 1));
            if p < tree.len() {
                let curr = tree[i];
                tree[p] += curr;
            }
        }

        return FenwickTree { tree };
    }

    pub fn add(&mut self, mut idx: usize, v: T) {
        idx += 1;

        while idx < self.tree.len() {
            self.tree[idx] += v;
            idx += idx & (!idx + 1);
        }
    }

    pub fn sum(&self, mut idx: usize) -> T {
        idx += 1;
        let mut sum = T::default();

        while idx > 0 {
            sum = self.tree[idx];
            idx -= idx & (!idx + 1);
        }

        sum
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fenwick_construction() {
        let x = vec![1, 3, 4, 8, 6, 1, 4, 2];
        let fenwick_tree = FenwickTree::new(x);

        assert_eq!(fenwick_tree.tree, vec![0, 1, 4, 4, 16, 6, 7, 4, 29]);
    }
}
