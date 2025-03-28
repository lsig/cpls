use std::mem::swap;

#[derive(Debug, Clone)]
pub struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        let parent = (0..size).collect();
        let size = vec![1; size];

        UnionFind { parent, size }
    }

    pub fn find_set(&mut self, v: usize) -> usize {
        if self.parent[v] != v {
            self.parent[v] = self.find_set(self.parent[v]);
        }
        self.parent[v]
    }

    pub fn union_set(&mut self, a: usize, b: usize) {
        let mut a = self.find_set(a);
        let mut b = self.find_set(b);

        if a != b {
            if self.size[a] < self.size[b] {
                swap(&mut a, &mut b);
            }
            self.parent[b] = a;
            self.size[a] += self.size[b];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialization() {
        let mut uf = UnionFind::new(5); // Elements 0, 1, 2, 3, 4

        // Each element should be its own representative initially
        assert_eq!(uf.find_set(0), 0);
        assert_eq!(uf.find_set(1), 1);
        assert_eq!(uf.find_set(2), 2);
        assert_eq!(uf.find_set(3), 3);
        assert_eq!(uf.find_set(4), 4);

        // Initially, elements should be in different sets (have different representatives)
        assert!(uf.find_set(0) != uf.find_set(1));
        assert!(uf.find_set(3) != uf.find_set(4));
    }

    #[test]
    fn test_simple_union() {
        let mut uf = UnionFind::new(5);
        uf.union_set(0, 1);

        // 0 and 1 should now be in the same set (have the same representative)
        let root01 = uf.find_set(0);
        assert_eq!(uf.find_set(1), root01);

        // 0 and 2 should still be in different sets
        assert!(uf.find_set(0) != uf.find_set(2));
        assert!(uf.find_set(1) != uf.find_set(2)); // Also check 1 vs 2

        // Check root of an unmerged element
        assert_eq!(uf.find_set(2), 2);
    }

    #[test]
    fn test_transitive_union() {
        let mut uf = UnionFind::new(5); // 0, 1, 2, 3, 4
        uf.union_set(0, 1);
        uf.union_set(2, 3);
        uf.union_set(1, 3); // Connects the two pairs via 1 and 3

        // All of 0, 1, 2, 3 should now be in the same set
        let root = uf.find_set(0); // Get the representative of the combined set
        assert_eq!(uf.find_set(1), root);
        assert_eq!(uf.find_set(2), root);
        assert_eq!(uf.find_set(3), root);

        // 4 should remain separate
        assert_ne!(uf.find_set(4), root);
        assert_eq!(uf.find_set(4), 4); // Its representative should still be itself
    }

    #[test]
    fn test_union_already_connected() {
        let mut uf = UnionFind::new(4); // 0, 1, 2, 3
        uf.union_set(0, 1);
        let root01_before = uf.find_set(0); // Representative before second union

        // Union elements already in the same set
        uf.union_set(1, 0);

        // Representative should not change
        let root01_after = uf.find_set(0);
        assert_eq!(root01_after, root01_before);
        assert_eq!(uf.find_set(1), root01_after); // 1 should still have the same root

        // Also test unioning an element with itself
        uf.union_set(2, 2);
        assert_eq!(uf.find_set(2), 2); // Should remain its own root

        // Ensure sets {0,1} and {2} are still separate
        assert!(uf.find_set(0) != uf.find_set(2));
    }

    #[test]
    fn test_union_by_size_result() {
        // Note: We can't directly test *how* the union-by-size happened without
        // accessing internal state or adding specific test helpers.
        // This test just verifies the *result* that elements end up connected.
        let mut uf = UnionFind::new(6); // Elements 0 to 5

        // Create set {0, 1, 2}
        uf.union_set(0, 1);
        uf.union_set(1, 2);

        // Create set {3, 4}
        uf.union_set(3, 4);

        // Sanity check roots are different before final union
        let root0_before = uf.find_set(0);
        let root3_before = uf.find_set(3);
        assert_ne!(root0_before, root3_before);

        // Union the two sets
        uf.union_set(0, 3);

        // Verify all elements {0, 1, 2, 3, 4} are now in the same set
        let final_root = uf.find_set(0); // Get the representative of the merged set
        assert_eq!(uf.find_set(1), final_root);
        assert_eq!(uf.find_set(2), final_root);
        assert_eq!(uf.find_set(3), final_root);
        assert_eq!(uf.find_set(4), final_root);

        // Element 5 should still be separate
        assert_ne!(uf.find_set(5), final_root);
        assert_eq!(uf.find_set(5), 5);
    }

    // --- Boundary Condition Tests ---
    // These rely on panic behavior, which is okay for testing boundaries

    #[test]
    #[should_panic]
    fn test_find_out_of_bounds() {
        let mut uf = UnionFind::new(3);
        uf.find_set(3); // Index 3 is out of bounds for size 3 (indices 0, 1, 2)
    }

    #[test]
    #[should_panic]
    fn test_union_out_of_bounds_a() {
        let mut uf = UnionFind::new(3);
        uf.union_set(3, 1); // Index 3 is out of bounds
    }

    #[test]
    #[should_panic]
    fn test_union_out_of_bounds_b() {
        let mut uf = UnionFind::new(3);
        uf.union_set(1, 3); // Index 3 is out of bounds
    }

    #[test]
    fn test_new_zero_size() {
        // Test creating a UnionFind with size 0
        let uf = UnionFind::new(0);
        // We can't really call find_set or union_set, but new() shouldn't panic
        assert!(uf.parent.is_empty());
        assert!(uf.size.is_empty());
    }

    #[test]
    fn test_new_one_size() {
        // Test creating a UnionFind with size 1
        let mut uf = UnionFind::new(1);
        assert_eq!(uf.find_set(0), 0);
        // Unioning with itself should do nothing
        uf.union_set(0, 0);
        assert_eq!(uf.find_set(0), 0);
    }
}
