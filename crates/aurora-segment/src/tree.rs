//! Segment tree implementation for range queries and point updates.

/// A segment tree supporting range sum queries and point updates.
///
/// The tree is stored as a flat array with 1-based indexing.
/// For `n` elements, the tree uses `4*n` space.
#[derive(Debug, Clone)]
pub struct SegmentTree {
    /// Internal tree storage.
    tree: Vec<i64>,
    /// Number of elements in the original array.
    n: usize,
    /// Total queries performed.
    queries: u64,
    /// Total updates performed.
    updates: u64,
}

impl SegmentTree {
    /// Create a new segment tree from the given data.
    pub fn from_slice(data: &[i64]) -> Self {
        let n = data.len();
        let tree = vec![0i64; 4 * n.max(1)];
        let mut st = Self {
            tree,
            n,
            queries: 0,
            updates: 0,
        };
        if !data.is_empty() {
            st.build(data, 1, 0, n - 1);
        }
        st
    }

    /// Create an empty segment tree with `n` zeros.
    pub fn new(n: usize) -> Self {
        Self::from_slice(&vec![0i64; n])
    }

    fn build(&mut self, data: &[i64], node: usize, start: usize, end: usize) {
        if start == end {
            self.tree[node] = data[start];
            return;
        }
        let mid = start + (end - start) / 2;
        self.build(data, 2 * node, start, mid);
        self.build(data, 2 * node + 1, mid + 1, end);
        self.tree[node] = self.tree[2 * node].saturating_add(self.tree[2 * node + 1]);
    }

    /// Point update: set element at `idx` to `val`.
    pub fn update(&mut self, idx: usize, val: i64) {
        assert!(idx < self.n, "index {} out of range (n={})", idx, self.n);
        self.updates = self.updates.saturating_add(1);
        self.update_internal(1, 0, self.n - 1, idx, val);
    }

    fn update_internal(
        &mut self,
        node: usize,
        start: usize,
        end: usize,
        idx: usize,
        val: i64,
    ) {
        if start == end {
            self.tree[node] = val;
            return;
        }
        let mid = start + (end - start) / 2;
        if idx <= mid {
            self.update_internal(2 * node, start, mid, idx, val);
        } else {
            self.update_internal(2 * node + 1, mid + 1, end, idx, val);
        }
        self.tree[node] = self.tree[2 * node].saturating_add(self.tree[2 * node + 1]);
    }

    /// Range sum query over [l, r] (inclusive).
    pub fn query(&mut self, l: usize, r: usize) -> i64 {
        assert!(l <= r && r < self.n, "range [{}, {}] out of bounds (n={})", l, r, self.n);
        self.queries = self.queries.saturating_add(1);
        self.query_internal(1, 0, self.n - 1, l, r)
    }

    fn query_internal(
        &self,
        node: usize,
        start: usize,
        end: usize,
        l: usize,
        r: usize,
    ) -> i64 {
        if r < start || end < l {
            return 0;
        }
        if l <= start && end <= r {
            return self.tree[node];
        }
        let mid = start + (end - start) / 2;
        let left = self.query_internal(2 * node, start, mid, l, r);
        let right = self.query_internal(2 * node + 1, mid + 1, end, l, r);
        left.saturating_add(right)
    }

    /// Get the value at a single index.
    pub fn get(&mut self, idx: usize) -> i64 {
        self.query(idx, idx)
    }

    /// Number of elements.
    pub fn len(&self) -> usize {
        self.n
    }

    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    /// Total sum of all elements.
    pub fn total_sum(&mut self) -> i64 {
        if self.n == 0 {
            return 0;
        }
        self.query(0, self.n - 1)
    }

    /// Total queries performed.
    pub fn total_queries(&self) -> u64 {
        self.queries
    }

    /// Total updates performed.
    pub fn total_updates(&self) -> u64 {
        self.updates
    }

    /// Find the minimum prefix sum index where prefix sum >= target.
    /// Returns `None` if no such index exists.
    pub fn find_prefix(&mut self, target: i64) -> Option<usize> {
        let mut sum = 0i64;
        for i in 0..self.n {
            sum = sum.saturating_add(self.get(i));
            if sum >= target {
                return Some(i);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_slice() {
        let st = SegmentTree::from_slice(&[1, 2, 3, 4, 5]);
        assert_eq!(st.len(), 5);
        assert!(!st.is_empty());
    }

    #[test]
    fn test_new_zeros() {
        let mut st = SegmentTree::new(5);
        assert_eq!(st.total_sum(), 0);
    }

    #[test]
    fn test_range_query() {
        let mut st = SegmentTree::from_slice(&[1, 3, 5, 7, 9, 11]);
        assert_eq!(st.query(0, 2), 9);   // 1+3+5
        assert_eq!(st.query(1, 4), 24);  // 3+5+7+9
        assert_eq!(st.query(0, 5), 36);  // total
    }

    #[test]
    fn test_single_element_query() {
        let mut st = SegmentTree::from_slice(&[10, 20, 30]);
        assert_eq!(st.get(0), 10);
        assert_eq!(st.get(1), 20);
        assert_eq!(st.get(2), 30);
    }

    #[test]
    fn test_point_update() {
        let mut st = SegmentTree::from_slice(&[1, 2, 3, 4, 5]);
        st.update(2, 10); // change 3 -> 10
        assert_eq!(st.get(2), 10);
        assert_eq!(st.query(0, 4), 22); // 1+2+10+4+5
    }

    #[test]
    fn test_multiple_updates() {
        let mut st = SegmentTree::from_slice(&[0, 0, 0, 0]);
        st.update(0, 5);
        st.update(1, 10);
        st.update(2, 15);
        st.update(3, 20);
        assert_eq!(st.total_sum(), 50);
    }

    #[test]
    fn test_single_element_tree() {
        let mut st = SegmentTree::from_slice(&[42]);
        assert_eq!(st.get(0), 42);
        assert_eq!(st.total_sum(), 42);
        st.update(0, 100);
        assert_eq!(st.get(0), 100);
    }

    #[test]
    fn test_negative_values() {
        let mut st = SegmentTree::from_slice(&[-5, 10, -3, 8]);
        assert_eq!(st.query(0, 3), 10); // -5+10-3+8
        assert_eq!(st.query(0, 1), 5);  // -5+10
    }

    #[test]
    fn test_total_sum() {
        let mut st = SegmentTree::from_slice(&[1, 2, 3, 4, 5]);
        assert_eq!(st.total_sum(), 15);
    }

    #[test]
    fn test_find_prefix() {
        let mut st = SegmentTree::from_slice(&[1, 2, 3, 4, 5]);
        assert_eq!(st.find_prefix(6), Some(2));  // 1+2+3=6
        assert_eq!(st.find_prefix(1), Some(0));  // 1>=1
        assert_eq!(st.find_prefix(100), None);
    }

    #[test]
    fn test_stats() {
        let mut st = SegmentTree::from_slice(&[1, 2, 3]);
        st.query(0, 2);
        st.query(0, 1);
        st.update(0, 5);
        assert_eq!(st.total_queries(), 2);
        assert_eq!(st.total_updates(), 1);
    }

    #[test]
    fn test_empty_tree() {
        let mut st = SegmentTree::new(0);
        assert!(st.is_empty());
        assert_eq!(st.total_sum(), 0);
    }

    #[test]
    fn test_large_values() {
        let mut st = SegmentTree::from_slice(&[i64::MAX / 2, i64::MAX / 2]);
        let sum = st.query(0, 1);
        assert!(sum > 0); // saturating add prevents overflow
    }

    #[test]
    #[should_panic]
    fn test_out_of_range_update() {
        let mut st = SegmentTree::from_slice(&[1, 2, 3]);
        st.update(5, 10);
    }

    #[test]
    #[should_panic]
    fn test_out_of_range_query() {
        let mut st = SegmentTree::from_slice(&[1, 2, 3]);
        st.query(0, 5);
    }
}
