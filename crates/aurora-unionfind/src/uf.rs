//! Union-Find (Disjoint Set Union) implementation with path compression
//! and union by rank.

/// A Union-Find data structure for tracking connected components.
///
/// Uses path compression and union by rank for near-constant-time operations.
#[derive(Debug, Clone)]
pub struct UnionFind {
    /// Parent array. `parent[i]` points to the parent of element `i`.
    parent: Vec<usize>,
    /// Rank array for union by rank.
    rank: Vec<u32>,
    /// Number of distinct components.
    components: usize,
    /// Total union operations performed.
    unions: u64,
    /// Total find operations performed.
    finds: u64,
}

impl UnionFind {
    /// Create a new Union-Find with `n` elements (0..n), each in its own set.
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
            components: n,
            unions: 0,
            finds: 0,
        }
    }

    /// Find the representative (root) of the set containing `x`.
    /// Uses path compression for amortized near-O(1).
    pub fn find(&mut self, x: usize) -> usize {
        assert!(x < self.parent.len(), "element {} out of range", x);
        self.finds = self.finds.saturating_add(1);
        self.find_internal(x)
    }

    fn find_internal(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find_internal(self.parent[x]);
        }
        self.parent[x]
    }

    /// Union the sets containing `x` and `y`.
    /// Returns `true` if they were in different sets (merge happened).
    /// Returns `false` if they were already in the same set.
    pub fn union(&mut self, x: usize, y: usize) -> bool {
        self.unions = self.unions.saturating_add(1);
        let rx = self.find_internal(x);
        let ry = self.find_internal(y);
        if rx == ry {
            return false;
        }
        // Union by rank
        match self.rank[rx].cmp(&self.rank[ry]) {
            std::cmp::Ordering::Less => self.parent[rx] = ry,
            std::cmp::Ordering::Greater => self.parent[ry] = rx,
            std::cmp::Ordering::Equal => {
                self.parent[ry] = rx;
                self.rank[rx] = self.rank[rx].saturating_add(1);
            }
        }
        self.components = self.components.saturating_sub(1);
        true
    }

    /// Check if `x` and `y` are in the same set.
    pub fn connected(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    /// Number of distinct components.
    pub fn components(&self) -> usize {
        self.components
    }

    /// Total number of elements.
    pub fn len(&self) -> usize {
        self.parent.len()
    }

    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.parent.is_empty()
    }

    /// Get the size of the component containing `x`.
    pub fn component_size(&mut self, x: usize) -> usize {
        let root = self.find(x);
        (0..self.parent.len())
            .filter(|&i| self.find_internal(i) == root)
            .count()
    }

    /// Get all elements in the same component as `x`.
    pub fn component_members(&mut self, x: usize) -> Vec<usize> {
        let root = self.find(x);
        (0..self.parent.len())
            .filter(|&i| self.find_internal(i) == root)
            .collect()
    }

    /// Total find operations.
    pub fn total_finds(&self) -> u64 {
        self.finds
    }

    /// Total union operations.
    pub fn total_unions(&self) -> u64 {
        self.unions
    }

    /// Reset all elements to their own individual sets.
    pub fn reset(&mut self) {
        let n = self.parent.len();
        self.parent = (0..n).collect();
        self.rank = vec![0; n];
        self.components = n;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let uf = UnionFind::new(5);
        assert_eq!(uf.components(), 5);
        assert_eq!(uf.len(), 5);
        assert!(!uf.is_empty());
    }

    #[test]
    fn test_find_self() {
        let mut uf = UnionFind::new(5);
        for i in 0..5 {
            assert_eq!(uf.find(i), i);
        }
    }

    #[test]
    fn test_union_basic() {
        let mut uf = UnionFind::new(5);
        assert!(uf.union(0, 1));
        assert_eq!(uf.components(), 4);
        assert!(uf.connected(0, 1));
    }

    #[test]
    fn test_union_already_connected() {
        let mut uf = UnionFind::new(5);
        uf.union(0, 1);
        assert!(!uf.union(0, 1)); // already same set
        assert_eq!(uf.components(), 4);
    }

    #[test]
    fn test_transitive() {
        let mut uf = UnionFind::new(5);
        uf.union(0, 1);
        uf.union(1, 2);
        assert!(uf.connected(0, 2));
        assert_eq!(uf.components(), 3);
    }

    #[test]
    fn test_not_connected() {
        let mut uf = UnionFind::new(5);
        uf.union(0, 1);
        assert!(!uf.connected(0, 3));
    }

    #[test]
    fn test_all_connected() {
        let mut uf = UnionFind::new(4);
        uf.union(0, 1);
        uf.union(2, 3);
        uf.union(0, 2);
        assert_eq!(uf.components(), 1);
        for i in 0..4 {
            for j in 0..4 {
                assert!(uf.connected(i, j));
            }
        }
    }

    #[test]
    fn test_component_size() {
        let mut uf = UnionFind::new(6);
        uf.union(0, 1);
        uf.union(1, 2);
        assert_eq!(uf.component_size(0), 3);
        assert_eq!(uf.component_size(3), 1);
    }

    #[test]
    fn test_component_members() {
        let mut uf = UnionFind::new(5);
        uf.union(0, 2);
        uf.union(2, 4);
        let mut members = uf.component_members(0);
        members.sort();
        assert_eq!(members, vec![0, 2, 4]);
    }

    #[test]
    fn test_reset() {
        let mut uf = UnionFind::new(5);
        uf.union(0, 1);
        uf.union(2, 3);
        uf.reset();
        assert_eq!(uf.components(), 5);
        assert!(!uf.connected(0, 1));
    }

    #[test]
    fn test_stats() {
        let mut uf = UnionFind::new(5);
        uf.find(0);
        uf.find(1);
        uf.union(0, 1);
        assert_eq!(uf.total_finds(), 2);
        assert_eq!(uf.total_unions(), 1);
    }

    #[test]
    fn test_empty() {
        let uf = UnionFind::new(0);
        assert!(uf.is_empty());
        assert_eq!(uf.components(), 0);
    }

    #[test]
    fn test_single_element() {
        let mut uf = UnionFind::new(1);
        assert_eq!(uf.find(0), 0);
        assert_eq!(uf.components(), 1);
    }

    #[test]
    #[should_panic]
    fn test_out_of_range() {
        let mut uf = UnionFind::new(3);
        uf.find(5);
    }

    #[test]
    fn test_path_compression() {
        let mut uf = UnionFind::new(100);
        // Chain: 0-1-2-...-99
        for i in 0..99 {
            uf.union(i, i + 1);
        }
        assert_eq!(uf.components(), 1);
        // After find with path compression, parent should point to root
        let root = uf.find(99);
        assert_eq!(uf.find(0), root);
        assert_eq!(uf.find(50), root);
    }
}
