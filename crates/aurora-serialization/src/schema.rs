//! Schema definition and versioning — describes data structures with version tracking.

use std::collections::HashMap;

/// Supported field types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldType {
    Bool,
    U8,
    U16,
    U32,
    U64,
    I32,
    I64,
    F32,
    F64,
    String,
    Bytes,
    Array,
    Map,
}

impl FieldType {
    /// Short name for display.
    pub fn as_str(&self) -> &'static str {
        match self {
            FieldType::Bool => "bool",
            FieldType::U8 => "u8",
            FieldType::U16 => "u16",
            FieldType::U32 => "u32",
            FieldType::U64 => "u64",
            FieldType::I32 => "i32",
            FieldType::I64 => "i64",
            FieldType::F32 => "f32",
            FieldType::F64 => "f64",
            FieldType::String => "string",
            FieldType::Bytes => "bytes",
            FieldType::Array => "array",
            FieldType::Map => "map",
        }
    }
}

/// A field in a schema.
#[derive(Debug, Clone)]
pub struct FieldDef {
    /// Field name.
    pub name: String,
    /// Field type.
    pub field_type: FieldType,
    /// Field tag number (for wire format).
    pub tag: u32,
    /// Whether the field is required.
    pub required: bool,
    /// Default value as bytes (empty if no default).
    pub default_value: Vec<u8>,
}

impl FieldDef {
    /// Create a required field.
    pub fn required(name: &str, field_type: FieldType, tag: u32) -> Self {
        Self {
            name: name.to_string(),
            field_type,
            tag,
            required: true,
            default_value: Vec::new(),
        }
    }

    /// Create an optional field.
    pub fn optional(name: &str, field_type: FieldType, tag: u32) -> Self {
        Self {
            name: name.to_string(),
            field_type,
            tag,
            required: false,
            default_value: Vec::new(),
        }
    }

    /// Set a default value (as raw bytes).
    pub fn with_default(mut self, default: Vec<u8>) -> Self {
        self.default_value = default;
        self
    }
}

/// A versioned schema for a data type.
#[derive(Debug, Clone)]
pub struct Schema {
    /// Type name.
    pub name: String,
    /// Version number.
    pub version: u32,
    /// Fields in this schema.
    pub fields: Vec<FieldDef>,
}

impl Schema {
    /// Create a new schema.
    pub fn new(name: &str, version: u32) -> Self {
        Self {
            name: name.to_string(),
            version,
            fields: Vec::new(),
        }
    }

    /// Add a field.
    pub fn add_field(&mut self, field: FieldDef) {
        self.fields.push(field);
    }

    /// Builder: add a field and return self.
    pub fn with_field(mut self, field: FieldDef) -> Self {
        self.fields.push(field);
        self
    }

    /// Get a field by name.
    pub fn field_by_name(&self, name: &str) -> Option<&FieldDef> {
        self.fields.iter().find(|f| f.name == name)
    }

    /// Get a field by tag.
    pub fn field_by_tag(&self, tag: u32) -> Option<&FieldDef> {
        self.fields.iter().find(|f| f.tag == tag)
    }

    /// Required fields.
    pub fn required_fields(&self) -> Vec<&FieldDef> {
        self.fields.iter().filter(|f| f.required).collect()
    }

    /// Number of fields.
    pub fn field_count(&self) -> usize {
        self.fields.len()
    }

    /// Compute a fingerprint of the schema (simple hash of name+version+fields).
    pub fn fingerprint(&self) -> u64 {
        let mut hash: u64 = 0xcbf29ce484222325; // FNV offset basis
        for byte in self.name.as_bytes() {
            hash ^= *byte as u64;
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash ^= self.version as u64;
        hash = hash.wrapping_mul(0x100000001b3);
        for field in &self.fields {
            hash ^= field.tag as u64;
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash
    }
}

/// Schema registry — stores multiple schema versions.
pub struct SchemaRegistry {
    /// Schemas indexed by (name, version).
    schemas: HashMap<(String, u32), Schema>,
    /// Latest version for each name.
    latest: HashMap<String, u32>,
}

impl SchemaRegistry {
    /// Create a new registry.
    pub fn new() -> Self {
        Self {
            schemas: HashMap::new(),
            latest: HashMap::new(),
        }
    }

    /// Register a schema.
    pub fn register(&mut self, schema: Schema) {
        let name = schema.name.clone();
        let version = schema.version;
        self.schemas.insert((name.clone(), version), schema);

        let entry = self.latest.entry(name).or_insert(0);
        if version > *entry {
            *entry = version;
        }
    }

    /// Get a specific version of a schema.
    pub fn get(&self, name: &str, version: u32) -> Option<&Schema> {
        self.schemas.get(&(name.to_string(), version))
    }

    /// Get the latest version of a schema.
    pub fn get_latest(&self, name: &str) -> Option<&Schema> {
        let version = self.latest.get(name)?;
        self.get(name, *version)
    }

    /// Latest version number for a schema name.
    pub fn latest_version(&self, name: &str) -> Option<u32> {
        self.latest.get(name).copied()
    }

    /// List all registered type names.
    pub fn type_names(&self) -> Vec<&str> {
        self.latest.keys().map(|s| s.as_str()).collect()
    }

    /// All versions of a given type.
    pub fn versions(&self, name: &str) -> Vec<u32> {
        let mut versions: Vec<u32> = self
            .schemas
            .keys()
            .filter(|(n, _)| n == name)
            .map(|(_, v)| *v)
            .collect();
        versions.sort();
        versions
    }

    /// Check compatibility between two versions (all v1 required fields exist in v2).
    pub fn is_forward_compatible(&self, name: &str, v1: u32, v2: u32) -> bool {
        let s1 = match self.get(name, v1) {
            Some(s) => s,
            None => return false,
        };
        let s2 = match self.get(name, v2) {
            Some(s) => s,
            None => return false,
        };

        // All required fields in v1 must exist in v2
        s1.required_fields()
            .iter()
            .all(|f| s2.field_by_tag(f.tag).is_some())
    }
}

impl Default for SchemaRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_schema_v1() -> Schema {
        Schema::new("Position", 1)
            .with_field(FieldDef::required("lat", FieldType::F64, 1))
            .with_field(FieldDef::required("lon", FieldType::F64, 2))
            .with_field(FieldDef::optional("alt", FieldType::F64, 3))
    }

    fn sample_schema_v2() -> Schema {
        Schema::new("Position", 2)
            .with_field(FieldDef::required("lat", FieldType::F64, 1))
            .with_field(FieldDef::required("lon", FieldType::F64, 2))
            .with_field(FieldDef::optional("alt", FieldType::F64, 3))
            .with_field(FieldDef::optional("accuracy", FieldType::F32, 4))
    }

    #[test]
    fn test_schema_creation() {
        let s = sample_schema_v1();
        assert_eq!(s.name, "Position");
        assert_eq!(s.version, 1);
        assert_eq!(s.field_count(), 3);
    }

    #[test]
    fn test_field_lookup_by_name() {
        let s = sample_schema_v1();
        let field = s.field_by_name("lat").unwrap();
        assert_eq!(field.field_type, FieldType::F64);
        assert!(field.required);
    }

    #[test]
    fn test_field_lookup_by_tag() {
        let s = sample_schema_v1();
        let field = s.field_by_tag(2).unwrap();
        assert_eq!(field.name, "lon");
    }

    #[test]
    fn test_required_fields() {
        let s = sample_schema_v1();
        let required = s.required_fields();
        assert_eq!(required.len(), 2);
    }

    #[test]
    fn test_fingerprint_deterministic() {
        let s1 = sample_schema_v1();
        let s2 = sample_schema_v1();
        assert_eq!(s1.fingerprint(), s2.fingerprint());
    }

    #[test]
    fn test_fingerprint_differs_by_version() {
        let s1 = sample_schema_v1();
        let s2 = sample_schema_v2();
        assert_ne!(s1.fingerprint(), s2.fingerprint());
    }

    #[test]
    fn test_registry_register_and_get() {
        let mut reg = SchemaRegistry::new();
        reg.register(sample_schema_v1());
        reg.register(sample_schema_v2());

        assert_eq!(reg.latest_version("Position"), Some(2));
        let latest = reg.get_latest("Position").unwrap();
        assert_eq!(latest.field_count(), 4);
    }

    #[test]
    fn test_registry_versions() {
        let mut reg = SchemaRegistry::new();
        reg.register(sample_schema_v1());
        reg.register(sample_schema_v2());

        let versions = reg.versions("Position");
        assert_eq!(versions, vec![1, 2]);
    }

    #[test]
    fn test_forward_compatibility() {
        let mut reg = SchemaRegistry::new();
        reg.register(sample_schema_v1());
        reg.register(sample_schema_v2());

        // v1 -> v2: all v1 required fields exist in v2
        assert!(reg.is_forward_compatible("Position", 1, 2));
    }

    #[test]
    fn test_field_default() {
        let field = FieldDef::optional("speed", FieldType::F64, 5)
            .with_default(0.0f64.to_le_bytes().to_vec());
        assert_eq!(field.default_value.len(), 8);
    }
}
