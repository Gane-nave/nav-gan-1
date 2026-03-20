use crate::bits::BitSet;

/// Perform bitwise AND of two bit sets, returning a new bit set.
/// The result has the capacity of the smaller set.
pub fn and(a: &BitSet, b: &BitSet) -> BitSet {
    let cap = a.capacity().min(b.capacity());
    let mut result = BitSet::new(cap);
    let min_words = a.words().len().min(b.words().len());
    for i in 0..min_words {
        let word = a.words()[i] & b.words()[i];
        set_word_bits(&mut result, i, word, cap);
    }
    result
}

/// Perform bitwise OR of two bit sets, returning a new bit set.
/// The result has the capacity of the larger set.
pub fn or(a: &BitSet, b: &BitSet) -> BitSet {
    let cap = a.capacity().max(b.capacity());
    let mut result = BitSet::new(cap);
    let max_words = a.words().len().max(b.words().len());
    for i in 0..max_words {
        let wa = if i < a.words().len() { a.words()[i] } else { 0 };
        let wb = if i < b.words().len() { b.words()[i] } else { 0 };
        let word = wa | wb;
        set_word_bits(&mut result, i, word, cap);
    }
    result
}

/// Perform bitwise XOR of two bit sets, returning a new bit set.
/// The result has the capacity of the larger set.
pub fn xor(a: &BitSet, b: &BitSet) -> BitSet {
    let cap = a.capacity().max(b.capacity());
    let mut result = BitSet::new(cap);
    let max_words = a.words().len().max(b.words().len());
    for i in 0..max_words {
        let wa = if i < a.words().len() { a.words()[i] } else { 0 };
        let wb = if i < b.words().len() { b.words()[i] } else { 0 };
        let word = wa ^ wb;
        set_word_bits(&mut result, i, word, cap);
    }
    result
}

/// Compute the difference (a AND NOT b), returning a new bit set.
pub fn difference(a: &BitSet, b: &BitSet) -> BitSet {
    let cap = a.capacity();
    let mut result = BitSet::new(cap);
    for i in 0..a.words().len() {
        let wb = if i < b.words().len() { b.words()[i] } else { 0 };
        let word = a.words()[i] & !wb;
        set_word_bits(&mut result, i, word, cap);
    }
    result
}

/// Check if a is a subset of b (every bit in a is also in b).
pub fn is_subset(a: &BitSet, b: &BitSet) -> bool {
    for i in 0..a.words().len() {
        let wb = if i < b.words().len() { b.words()[i] } else { 0 };
        if a.words()[i] & !wb != 0 {
            return false;
        }
    }
    true
}

/// Count the number of bits in common (population count of AND).
pub fn intersection_count(a: &BitSet, b: &BitSet) -> usize {
    let min_words = a.words().len().min(b.words().len());
    let mut count = 0usize;
    for i in 0..min_words {
        count += (a.words()[i] & b.words()[i]).count_ones() as usize;
    }
    count
}

/// Compute the Jaccard similarity between two bit sets.
/// Returns 0.0 if both sets are empty.
pub fn jaccard(a: &BitSet, b: &BitSet) -> f64 {
    let max_words = a.words().len().max(b.words().len());
    let mut inter = 0u64;
    let mut union = 0u64;
    for i in 0..max_words {
        let wa = if i < a.words().len() { a.words()[i] } else { 0 };
        let wb = if i < b.words().len() { b.words()[i] } else { 0 };
        inter += (wa & wb).count_ones() as u64;
        union += (wa | wb).count_ones() as u64;
    }
    if union == 0 {
        0.0
    } else {
        inter as f64 / union as f64
    }
}

/// Helper to set individual bits from a word into a BitSet.
fn set_word_bits(bs: &mut BitSet, word_idx: usize, word: u64, cap: usize) {
    let mut w = word;
    while w != 0 {
        let bit = w.trailing_zeros() as usize;
        let index = word_idx * 64 + bit;
        if index < cap {
            bs.set(index);
        }
        w &= w - 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_set(cap: usize, indices: &[usize]) -> BitSet {
        let mut bs = BitSet::new(cap);
        for &i in indices {
            bs.set(i);
        }
        bs
    }

    #[test]
    fn test_and() {
        let a = make_set(100, &[1, 2, 3, 5, 10]);
        let b = make_set(100, &[2, 3, 4, 10, 20]);
        let r = and(&a, &b);
        assert_eq!(r.iter_set(), vec![2, 3, 10]);
    }

    #[test]
    fn test_or() {
        let a = make_set(64, &[0, 1, 2]);
        let b = make_set(64, &[2, 3, 4]);
        let r = or(&a, &b);
        assert_eq!(r.iter_set(), vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_xor() {
        let a = make_set(64, &[0, 1, 2]);
        let b = make_set(64, &[1, 2, 3]);
        let r = xor(&a, &b);
        assert_eq!(r.iter_set(), vec![0, 3]);
    }

    #[test]
    fn test_difference() {
        let a = make_set(64, &[1, 2, 3, 4]);
        let b = make_set(64, &[2, 4]);
        let r = difference(&a, &b);
        assert_eq!(r.iter_set(), vec![1, 3]);
    }

    #[test]
    fn test_is_subset() {
        let a = make_set(64, &[1, 2]);
        let b = make_set(64, &[0, 1, 2, 3]);
        assert!(is_subset(&a, &b));
        assert!(!is_subset(&b, &a));
    }

    #[test]
    fn test_intersection_count() {
        let a = make_set(128, &[0, 10, 64, 100]);
        let b = make_set(128, &[10, 50, 64, 127]);
        assert_eq!(intersection_count(&a, &b), 2); // 10, 64
    }

    #[test]
    fn test_jaccard() {
        let a = make_set(64, &[0, 1, 2, 3]);
        let b = make_set(64, &[2, 3, 4, 5]);
        let j = jaccard(&a, &b);
        // intersection = {2,3} = 2, union = {0,1,2,3,4,5} = 6
        assert!((j - 2.0 / 6.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_jaccard_empty() {
        let a = BitSet::new(64);
        let b = BitSet::new(64);
        assert_eq!(jaccard(&a, &b), 0.0);
    }

    #[test]
    fn test_and_different_sizes() {
        let a = make_set(128, &[0, 65, 100]);
        let b = make_set(64, &[0, 10]);
        let r = and(&a, &b);
        assert_eq!(r.iter_set(), vec![0]); // only bit 0 in common within smaller cap
    }

    #[test]
    fn test_or_different_sizes() {
        let a = make_set(64, &[0, 1]);
        let b = make_set(128, &[1, 100]);
        let r = or(&a, &b);
        assert_eq!(r.capacity(), 128);
        assert!(r.test(0));
        assert!(r.test(1));
        assert!(r.test(100));
    }
}
