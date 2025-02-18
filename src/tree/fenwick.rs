use std::{iter::once, ops::AddAssign};

#[derive(Debug, Clone)]
pub struct FenwickTree<T: AddAssign + Default + Copy + PartialEq> {
    tree: Vec<T>,
}

// Tree is zero-indexed
impl<T: AddAssign + Default + Copy + PartialEq> FenwickTree<T> {
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
            sum += self.tree[idx];
            idx -= idx & (!idx + 1);
        }

        sum
    }

    pub fn is_empty(&self) -> bool {
        self.tree.is_empty()
    }

    // Maintain zero-index from outside
    pub fn len(&self) -> usize {
        if self.is_empty() {
            0
        } else {
            self.tree.len() - 1
        }
    }
}

impl<T: AddAssign + Default + Copy + PartialEq> PartialEq<Vec<T>> for FenwickTree<T> {
    fn eq(&self, other: &Vec<T>) -> bool {
        self.tree[1..] == *other
    }
}

impl<T: AddAssign + Default + Copy + PartialEq> PartialEq for FenwickTree<T> {
    fn eq(&self, other: &Self) -> bool {
        self.tree == other.tree
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fenwick_construction() {
        let x = vec![1, 3, 4, 8, 6, 1, 4, 2];
        let fenwick_tree = FenwickTree::new(x);
        let tree2 = fenwick_tree.clone();

        assert_eq!(fenwick_tree, vec![1, 4, 4, 16, 6, 7, 4, 29]);
        assert_eq!(fenwick_tree, tree2);
    }

    #[test]
    fn test_fenwick_sum() {
        let x = vec![1, 3, 4, 8, 6, 1, 4, 2];
        let fenwick_tree = FenwickTree::new(x);

        assert_eq!(fenwick_tree.sum(6), 27);
    }
}
