//! Rope data structure for efficient string manipulation.

/// A rope for efficient insertion, deletion, and concatenation of strings.
///
/// Uses a balanced tree of string chunks. This simplified implementation
/// uses a vector of chunks with configurable chunk size.
#[derive(Debug, Clone)]
pub struct Rope {
    /// Chunks of the rope.
    chunks: Vec<String>,
    /// Maximum chunk size before splitting.
    chunk_size: usize,
    /// Total length in bytes.
    total_len: usize,
    /// Total edits performed.
    edits: u64,
}

impl Rope {
    /// Create a new empty rope with the given chunk size.
    pub fn new(chunk_size: usize) -> Self {
        assert!(chunk_size > 0, "chunk_size must be > 0");
        Self {
            chunks: Vec::new(),
            chunk_size,
            total_len: 0,
            edits: 0,
        }
    }

    /// Create a rope from a string.
    pub fn from_str(s: &str, chunk_size: usize) -> Self {
        assert!(chunk_size > 0, "chunk_size must be > 0");
        let mut rope = Self {
            chunks: Vec::new(),
            chunk_size,
            total_len: 0,
            edits: 0,
        };
        if !s.is_empty() {
            // Split into chunks
            let bytes = s.as_bytes();
            let mut start = 0;
            while start < bytes.len() {
                let end = (start + chunk_size).min(bytes.len());
                // Ensure we don't split in the middle of a UTF-8 character
                let end = Self::adjust_boundary(s, end);
                rope.chunks.push(s[start..end].to_string());
                start = end;
            }
            rope.total_len = s.len();
        }
        rope
    }

    /// Adjust a byte boundary to not split a UTF-8 character.
    fn adjust_boundary(s: &str, pos: usize) -> usize {
        if pos >= s.len() {
            return s.len();
        }
        let mut p = pos;
        while p > 0 && !s.is_char_boundary(p) {
            p -= 1;
        }
        if p == 0 && !s.is_char_boundary(p) {
            s.len()
        } else {
            p
        }
    }

    /// Insert a string at the given byte position.
    pub fn insert(&mut self, pos: usize, s: &str) {
        assert!(pos <= self.total_len, "position {} out of bounds (len={})", pos, self.total_len);
        self.edits = self.edits.saturating_add(1);

        if self.chunks.is_empty() {
            self.chunks.push(s.to_string());
            self.total_len = s.len();
            return;
        }

        // Find the chunk and offset within it
        let (chunk_idx, offset) = self.find_chunk(pos);

        if chunk_idx >= self.chunks.len() {
            // Append at end
            self.chunks.push(s.to_string());
        } else {
            let chunk = &self.chunks[chunk_idx];
            let mut new_chunk = String::with_capacity(chunk.len() + s.len());
            new_chunk.push_str(&chunk[..offset]);
            new_chunk.push_str(s);
            new_chunk.push_str(&chunk[offset..]);
            self.chunks[chunk_idx] = new_chunk;
        }

        self.total_len += s.len();
        self.rebalance_chunk(chunk_idx.min(self.chunks.len() - 1));
    }

    /// Delete bytes in the range [start, end).
    pub fn delete(&mut self, start: usize, end: usize) {
        assert!(start <= end, "start ({}) must be <= end ({})", start, end);
        assert!(end <= self.total_len, "end ({}) out of bounds (len={})", end, self.total_len);
        self.edits = self.edits.saturating_add(1);

        if start == end {
            return;
        }

        // Rebuild string, remove range, re-chunk
        let full = self.collect_string();
        let new_str = format!("{}{}", &full[..start], &full[end..]);
        let edits = self.edits;
        let chunk_size = self.chunk_size;
        *self = Self::from_str(&new_str, chunk_size);
        self.edits = edits;
    }

    /// Append a string to the end.
    pub fn append(&mut self, s: &str) {
        let pos = self.total_len;
        self.insert(pos, s);
    }

    /// Get the full string content.
    pub fn collect_string(&self) -> String {
        self.chunks.join("")
    }

    /// Get a substring by byte range.
    pub fn slice(&self, start: usize, end: usize) -> String {
        let full = self.collect_string();
        full[start..end].to_string()
    }

    /// Get the byte at a position.
    pub fn byte_at(&self, pos: usize) -> u8 {
        assert!(pos < self.total_len, "position {} out of bounds", pos);
        let (chunk_idx, offset) = self.find_chunk(pos);
        self.chunks[chunk_idx].as_bytes()[offset]
    }

    /// Total length in bytes.
    pub fn len(&self) -> usize {
        self.total_len
    }

    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.total_len == 0
    }

    /// Number of chunks.
    pub fn chunk_count(&self) -> usize {
        self.chunks.len()
    }

    /// Total edits performed.
    pub fn total_edits(&self) -> u64 {
        self.edits
    }

    /// Find which chunk contains the given byte position.
    /// Returns (chunk_index, offset_within_chunk).
    fn find_chunk(&self, pos: usize) -> (usize, usize) {
        let mut remaining = pos;
        for (i, chunk) in self.chunks.iter().enumerate() {
            if remaining <= chunk.len() {
                return (i, remaining);
            }
            remaining -= chunk.len();
        }
        (self.chunks.len(), 0)
    }

    /// Split a chunk if it's too large.
    fn rebalance_chunk(&mut self, idx: usize) {
        if idx >= self.chunks.len() {
            return;
        }
        if self.chunks[idx].len() > self.chunk_size * 2 {
            let chunk = self.chunks[idx].clone();
            let mid = Self::adjust_boundary(&chunk, chunk.len() / 2);
            let left = chunk[..mid].to_string();
            let right = chunk[mid..].to_string();
            self.chunks[idx] = left;
            self.chunks.insert(idx + 1, right);
        }
    }

    /// Get the character count (not byte count).
    pub fn char_count(&self) -> usize {
        self.chunks.iter().map(|c| c.chars().count()).sum()
    }

    /// Search for a substring. Returns the byte offset or None.
    pub fn find(&self, needle: &str) -> Option<usize> {
        self.collect_string().find(needle)
    }

    /// Replace all occurrences of `from` with `to`.
    pub fn replace_all(&mut self, from: &str, to: &str) {
        let full = self.collect_string().replace(from, to);
        let chunk_size = self.chunk_size;
        *self = Self::from_str(&full, chunk_size);
        self.edits = self.edits.saturating_add(1);
    }

    /// Split the rope at a byte position into two ropes.
    pub fn split_at(&self, pos: usize) -> (Rope, Rope) {
        let full = self.collect_string();
        let left = Self::from_str(&full[..pos], self.chunk_size);
        let right = Self::from_str(&full[pos..], self.chunk_size);
        (left, right)
    }

    /// Concatenate two ropes.
    pub fn concat(a: &Rope, b: &Rope) -> Rope {
        let mut result = a.clone();
        result.append(&b.collect_string());
        result
    }

    /// Clear all content.
    pub fn clear(&mut self) {
        self.chunks.clear();
        self.total_len = 0;
    }
}

impl std::fmt::Display for Rope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for chunk in &self.chunks {
            f.write_str(chunk)?;
        }
        Ok(())
    }
}

impl Default for Rope {
    fn default() -> Self {
        Self::new(1024)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_empty() {
        let rope = Rope::new(64);
        assert!(rope.is_empty());
        assert_eq!(rope.len(), 0);
        assert_eq!(rope.chunk_count(), 0);
    }

    #[test]
    fn test_from_str() {
        let rope = Rope::from_str("hello world", 64);
        assert_eq!(rope.to_string(), "hello world");
        assert_eq!(rope.collect_string(), "hello world");
        assert_eq!(rope.len(), 11);
    }

    #[test]
    fn test_from_str_chunked() {
        let rope = Rope::from_str("abcdefghij", 3);
        assert_eq!(rope.collect_string(), "abcdefghij");
        assert!(rope.chunk_count() > 1);
    }

    #[test]
    fn test_insert_beginning() {
        let mut rope = Rope::from_str("world", 64);
        rope.insert(0, "hello ");
        assert_eq!(rope.collect_string(), "hello world");
    }

    #[test]
    fn test_insert_middle() {
        let mut rope = Rope::from_str("helloworld", 64);
        rope.insert(5, " ");
        assert_eq!(rope.collect_string(), "hello world");
    }

    #[test]
    fn test_insert_end() {
        let mut rope = Rope::from_str("hello", 64);
        rope.insert(5, " world");
        assert_eq!(rope.collect_string(), "hello world");
    }

    #[test]
    fn test_delete() {
        let mut rope = Rope::from_str("hello world", 64);
        rope.delete(5, 11); // delete " world"
        assert_eq!(rope.collect_string(), "hello");
    }

    #[test]
    fn test_append() {
        let mut rope = Rope::from_str("hello", 64);
        rope.append(" world");
        assert_eq!(rope.collect_string(), "hello world");
    }

    #[test]
    fn test_slice() {
        let rope = Rope::from_str("hello world", 64);
        assert_eq!(rope.slice(0, 5), "hello");
        assert_eq!(rope.slice(6, 11), "world");
    }

    #[test]
    fn test_byte_at() {
        let rope = Rope::from_str("abc", 64);
        assert_eq!(rope.byte_at(0), b'a');
        assert_eq!(rope.byte_at(1), b'b');
        assert_eq!(rope.byte_at(2), b'c');
    }

    #[test]
    fn test_find() {
        let rope = Rope::from_str("hello world", 64);
        assert_eq!(rope.find("world"), Some(6));
        assert_eq!(rope.find("xyz"), None);
    }

    #[test]
    fn test_replace_all() {
        let mut rope = Rope::from_str("aabaa", 64);
        rope.replace_all("a", "x");
        assert_eq!(rope.collect_string(), "xxbxx");
    }

    #[test]
    fn test_split_at() {
        let rope = Rope::from_str("hello world", 64);
        let (left, right) = rope.split_at(5);
        assert_eq!(left.collect_string(), "hello");
        assert_eq!(right.collect_string(), " world");
    }

    #[test]
    fn test_concat() {
        let a = Rope::from_str("hello", 64);
        let b = Rope::from_str(" world", 64);
        let c = Rope::concat(&a, &b);
        assert_eq!(c.collect_string(), "hello world");
    }

    #[test]
    fn test_char_count() {
        let rope = Rope::from_str("hello", 64);
        assert_eq!(rope.char_count(), 5);
    }

    #[test]
    fn test_clear() {
        let mut rope = Rope::from_str("hello", 64);
        rope.clear();
        assert!(rope.is_empty());
    }

    #[test]
    fn test_default() {
        let rope = Rope::default();
        assert!(rope.is_empty());
    }

    #[test]
    fn test_edits_counter() {
        let mut rope = Rope::from_str("hello", 64);
        rope.insert(5, " world");
        rope.delete(0, 6);
        assert_eq!(rope.total_edits(), 2);
    }

    #[test]
    fn test_insert_into_empty() {
        let mut rope = Rope::new(64);
        rope.insert(0, "hello");
        assert_eq!(rope.collect_string(), "hello");
    }

    #[test]
    #[should_panic]
    fn test_insert_out_of_bounds() {
        let mut rope = Rope::from_str("abc", 64);
        rope.insert(10, "x");
    }

    #[test]
    #[should_panic]
    fn test_delete_out_of_bounds() {
        let mut rope = Rope::from_str("abc", 64);
        rope.delete(0, 10);
    }
}
