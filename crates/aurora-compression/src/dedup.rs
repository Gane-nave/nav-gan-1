//! Data deduplication — content-addressable storage with hash-based dedup.

use std::collections::HashMap;

/// Content hash (simplified — uses a fast non-cryptographic hash).
pub type ContentHash = u64;

/// Compute a simple hash for content (FNV-1a).
pub fn hash_content(data: &[u8]) -> ContentHash {
    let mut hash: u64 = 0xcbf29ce484222325;
    for &byte in data {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

/// A chunk of data stored in the dedup store.
#[derive(Debug, Clone)]
struct StoredChunk {
    data: Vec<u8>,
    ref_count: u32,
}

/// Content-addressable dedup store.
pub struct DedupStore {
    chunks: HashMap<ContentHash, StoredChunk>,
    /// Total bytes stored (logical, before dedup).
    total_logical: usize,
    /// Total bytes stored (physical, after dedup).
    total_physical: usize,
}

impl DedupStore {
    /// Create a new dedup store.
    pub fn new() -> Self {
        Self {
            chunks: HashMap::new(),
            total_logical: 0,
            total_physical: 0,
        }
    }

    /// Store data, deduplicating against existing content.
    /// Returns the content hash.
    pub fn store(&mut self, data: &[u8]) -> ContentHash {
        let hash = hash_content(data);
        self.total_logical += data.len();

        if let Some(chunk) = self.chunks.get_mut(&hash) {
            chunk.ref_count += 1;
        } else {
            self.total_physical += data.len();
            self.chunks.insert(
                hash,
                StoredChunk {
                    data: data.to_vec(),
                    ref_count: 1,
                },
            );
        }
        hash
    }

    /// Retrieve data by hash.
    pub fn get(&self, hash: ContentHash) -> Option<&[u8]> {
        self.chunks.get(&hash).map(|c| c.data.as_slice())
    }

    /// Check if a hash exists in the store.
    pub fn contains(&self, hash: ContentHash) -> bool {
        self.chunks.contains_key(&hash)
    }

    /// Remove a reference to a chunk. If ref count hits 0, delete it.
    /// Returns true if the chunk was fully removed.
    pub fn release(&mut self, hash: ContentHash) -> bool {
        if let Some(chunk) = self.chunks.get_mut(&hash) {
            chunk.ref_count -= 1;
            if chunk.ref_count == 0 {
                self.total_physical -= chunk.data.len();
                self.chunks.remove(&hash);
                return true;
            }
        }
        false
    }

    /// Reference count for a hash.
    pub fn ref_count(&self, hash: ContentHash) -> u32 {
        self.chunks.get(&hash).map_or(0, |c| c.ref_count)
    }

    /// Number of unique chunks stored.
    pub fn unique_chunks(&self) -> usize {
        self.chunks.len()
    }

    /// Deduplication ratio (1.0 = no dedup, higher = better dedup).
    pub fn dedup_ratio(&self) -> f64 {
        if self.total_physical == 0 {
            return 1.0;
        }
        self.total_logical as f64 / self.total_physical as f64
    }

    /// Total logical bytes stored.
    pub fn logical_size(&self) -> usize {
        self.total_logical
    }

    /// Total physical bytes stored.
    pub fn physical_size(&self) -> usize {
        self.total_physical
    }

    /// Clear all stored data.
    pub fn clear(&mut self) {
        self.chunks.clear();
        self.total_logical = 0;
        self.total_physical = 0;
    }
}

impl Default for DedupStore {
    fn default() -> Self {
        Self::new()
    }
}

/// Split data into fixed-size chunks for dedup.
pub fn chunk_fixed(data: &[u8], chunk_size: usize) -> Vec<&[u8]> {
    if chunk_size == 0 {
        return Vec::new();
    }
    data.chunks(chunk_size).collect()
}

/// Store data in chunks, returning list of hashes.
pub fn store_chunked(store: &mut DedupStore, data: &[u8], chunk_size: usize) -> Vec<ContentHash> {
    let chunks = chunk_fixed(data, chunk_size);
    chunks.iter().map(|chunk| store.store(chunk)).collect()
}

/// Retrieve chunked data by hash list.
pub fn retrieve_chunked(store: &DedupStore, hashes: &[ContentHash]) -> Option<Vec<u8>> {
    let mut result = Vec::new();
    for &hash in hashes {
        let chunk = store.get(hash)?;
        result.extend_from_slice(chunk);
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_deterministic() {
        let data = b"hello world";
        assert_eq!(hash_content(data), hash_content(data));
    }

    #[test]
    fn test_hash_different() {
        assert_ne!(hash_content(b"hello"), hash_content(b"world"));
    }

    #[test]
    fn test_store_and_get() {
        let mut store = DedupStore::new();
        let hash = store.store(b"test data");
        assert_eq!(store.get(hash), Some(b"test data".as_ref()));
        assert!(store.contains(hash));
    }

    #[test]
    fn test_deduplication() {
        let mut store = DedupStore::new();
        let h1 = store.store(b"duplicate");
        let h2 = store.store(b"duplicate");
        assert_eq!(h1, h2);
        assert_eq!(store.unique_chunks(), 1);
        assert_eq!(store.ref_count(h1), 2);
        assert!(store.dedup_ratio() > 1.5);
    }

    #[test]
    fn test_release() {
        let mut store = DedupStore::new();
        let hash = store.store(b"data");
        store.store(b"data"); // ref_count = 2

        assert!(!store.release(hash)); // ref_count = 1
        assert!(store.contains(hash));

        assert!(store.release(hash)); // ref_count = 0, removed
        assert!(!store.contains(hash));
    }

    #[test]
    fn test_chunked_storage() {
        let mut store = DedupStore::new();
        let data = b"aabbccddaabb"; // "aabb" appears twice
        let hashes = store_chunked(&mut store, data, 4);
        assert_eq!(hashes.len(), 3);
        assert_eq!(hashes[0], hashes[2]); // "aabb" deduplicated

        let retrieved = retrieve_chunked(&store, &hashes).unwrap();
        assert_eq!(retrieved, data);
    }

    #[test]
    fn test_empty_chunk_size() {
        let chunks = chunk_fixed(b"data", 0);
        assert!(chunks.is_empty());
    }

    #[test]
    fn test_clear() {
        let mut store = DedupStore::new();
        store.store(b"a");
        store.store(b"b");
        store.clear();
        assert_eq!(store.unique_chunks(), 0);
        assert_eq!(store.logical_size(), 0);
        assert_eq!(store.physical_size(), 0);
    }

    #[test]
    fn test_dedup_ratio_no_data() {
        let store = DedupStore::new();
        assert!((store.dedup_ratio() - 1.0).abs() < f64::EPSILON);
    }
}
