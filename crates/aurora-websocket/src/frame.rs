//! WebSocket message framing — opcodes, frame construction, and parsing.

/// WebSocket frame opcodes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opcode {
    /// UTF-8 text message.
    Text,
    /// Binary message.
    Binary,
    /// Connection close.
    Close,
    /// Ping (heartbeat request).
    Ping,
    /// Pong (heartbeat response).
    Pong,
    /// Continuation frame for fragmented messages.
    Continuation,
}

impl Opcode {
    /// Whether this is a control frame.
    pub fn is_control(&self) -> bool {
        matches!(self, Opcode::Close | Opcode::Ping | Opcode::Pong)
    }

    /// Whether this is a data frame.
    pub fn is_data(&self) -> bool {
        matches!(self, Opcode::Text | Opcode::Binary | Opcode::Continuation)
    }

    /// Numeric representation.
    pub fn as_u8(&self) -> u8 {
        match self {
            Opcode::Continuation => 0x0,
            Opcode::Text => 0x1,
            Opcode::Binary => 0x2,
            Opcode::Close => 0x8,
            Opcode::Ping => 0x9,
            Opcode::Pong => 0xA,
        }
    }

    /// Parse from numeric value.
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0x0 => Some(Opcode::Continuation),
            0x1 => Some(Opcode::Text),
            0x2 => Some(Opcode::Binary),
            0x8 => Some(Opcode::Close),
            0x9 => Some(Opcode::Ping),
            0xA => Some(Opcode::Pong),
            _ => None,
        }
    }
}

/// Close status codes per RFC 6455.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CloseCode {
    /// Normal closure.
    Normal,
    /// Endpoint going away.
    GoingAway,
    /// Protocol error.
    ProtocolError,
    /// Unsupported data type.
    Unsupported,
    /// No status code present.
    NoStatus,
    /// Abnormal closure (no close frame).
    Abnormal,
    /// Invalid payload data.
    InvalidData,
    /// Policy violation.
    PolicyViolation,
    /// Message too big.
    MessageTooBig,
    /// Server error.
    InternalError,
}

impl CloseCode {
    /// Numeric code.
    pub fn as_u16(&self) -> u16 {
        match self {
            CloseCode::Normal => 1000,
            CloseCode::GoingAway => 1001,
            CloseCode::ProtocolError => 1002,
            CloseCode::Unsupported => 1003,
            CloseCode::NoStatus => 1005,
            CloseCode::Abnormal => 1006,
            CloseCode::InvalidData => 1007,
            CloseCode::PolicyViolation => 1008,
            CloseCode::MessageTooBig => 1009,
            CloseCode::InternalError => 1011,
        }
    }

    /// Parse from numeric value.
    pub fn from_u16(code: u16) -> Option<Self> {
        match code {
            1000 => Some(CloseCode::Normal),
            1001 => Some(CloseCode::GoingAway),
            1002 => Some(CloseCode::ProtocolError),
            1003 => Some(CloseCode::Unsupported),
            1005 => Some(CloseCode::NoStatus),
            1006 => Some(CloseCode::Abnormal),
            1007 => Some(CloseCode::InvalidData),
            1008 => Some(CloseCode::PolicyViolation),
            1009 => Some(CloseCode::MessageTooBig),
            1011 => Some(CloseCode::InternalError),
            _ => None,
        }
    }
}

/// A WebSocket frame.
#[derive(Debug, Clone)]
pub struct Frame {
    /// Whether this is the final fragment.
    pub fin: bool,
    /// Frame opcode.
    pub opcode: Opcode,
    /// Payload data.
    pub payload: Vec<u8>,
    /// Whether the payload is masked.
    pub masked: bool,
    /// Masking key (4 bytes), if masked.
    pub mask_key: Option<[u8; 4]>,
}

impl Frame {
    /// Create a text frame.
    pub fn text(data: &str) -> Self {
        Self {
            fin: true,
            opcode: Opcode::Text,
            payload: data.as_bytes().to_vec(),
            masked: false,
            mask_key: None,
        }
    }

    /// Create a binary frame.
    pub fn binary(data: Vec<u8>) -> Self {
        Self {
            fin: true,
            opcode: Opcode::Binary,
            payload: data,
            masked: false,
            mask_key: None,
        }
    }

    /// Create a close frame.
    pub fn close(code: CloseCode, reason: &str) -> Self {
        let mut payload = Vec::new();
        let code_val = code.as_u16();
        payload.push((code_val >> 8) as u8);
        payload.push((code_val & 0xFF) as u8);
        payload.extend_from_slice(reason.as_bytes());
        Self {
            fin: true,
            opcode: Opcode::Close,
            payload,
            masked: false,
            mask_key: None,
        }
    }

    /// Create a ping frame.
    pub fn ping(data: &[u8]) -> Self {
        Self {
            fin: true,
            opcode: Opcode::Ping,
            payload: data.to_vec(),
            masked: false,
            mask_key: None,
        }
    }

    /// Create a pong frame (echo back ping data).
    pub fn pong(data: &[u8]) -> Self {
        Self {
            fin: true,
            opcode: Opcode::Pong,
            payload: data.to_vec(),
            masked: false,
            mask_key: None,
        }
    }

    /// Payload length in bytes.
    pub fn payload_len(&self) -> usize {
        self.payload.len()
    }

    /// Apply masking to the payload using the mask key.
    pub fn apply_mask(&mut self) {
        if let Some(key) = self.mask_key {
            for (i, byte) in self.payload.iter_mut().enumerate() {
                *byte ^= key[i % 4];
            }
        }
    }

    /// Set the mask key and mark as masked.
    pub fn set_mask(&mut self, key: [u8; 4]) {
        self.mask_key = Some(key);
        self.masked = true;
        self.apply_mask();
    }

    /// Serialize frame to bytes (simplified, no actual WebSocket wire format).
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        let mut first_byte = self.opcode.as_u8();
        if self.fin {
            first_byte |= 0x80;
        }
        buf.push(first_byte);

        // Length encoding
        let len = self.payload.len();
        if len < 126 {
            let mut second_byte = len as u8;
            if self.masked {
                second_byte |= 0x80;
            }
            buf.push(second_byte);
        } else if len < 65536 {
            let mut second_byte = 126u8;
            if self.masked {
                second_byte |= 0x80;
            }
            buf.push(second_byte);
            buf.push((len >> 8) as u8);
            buf.push((len & 0xFF) as u8);
        } else {
            let mut second_byte = 127u8;
            if self.masked {
                second_byte |= 0x80;
            }
            buf.push(second_byte);
            for i in (0..8).rev() {
                buf.push(((len >> (i * 8)) & 0xFF) as u8);
            }
        }

        if let Some(key) = self.mask_key {
            buf.extend_from_slice(&key);
        }
        buf.extend_from_slice(&self.payload);
        buf
    }
}

/// Assemble fragmented frames into a complete message.
pub struct FragmentAssembler {
    fragments: Vec<Frame>,
    first_opcode: Option<Opcode>,
}

impl FragmentAssembler {
    /// Create a new fragment assembler.
    pub fn new() -> Self {
        Self {
            fragments: Vec::new(),
            first_opcode: None,
        }
    }

    /// Add a frame. Returns the assembled payload if this was the final fragment.
    pub fn add(&mut self, frame: Frame) -> Option<(Opcode, Vec<u8>)> {
        if frame.opcode != Opcode::Continuation && self.fragments.is_empty() {
            self.first_opcode = Some(frame.opcode);
        }

        let is_fin = frame.fin;
        self.fragments.push(frame);

        if is_fin {
            let opcode = self.first_opcode.unwrap_or(Opcode::Binary);
            let payload: Vec<u8> = self.fragments.drain(..).flat_map(|f| f.payload).collect();
            self.first_opcode = None;
            Some((opcode, payload))
        } else {
            None
        }
    }

    /// Whether assembly is in progress.
    pub fn is_assembling(&self) -> bool {
        !self.fragments.is_empty()
    }

    /// Reset the assembler, discarding any partial fragments.
    pub fn reset(&mut self) {
        self.fragments.clear();
        self.first_opcode = None;
    }
}

impl Default for FragmentAssembler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opcode_roundtrip() {
        for op in [
            Opcode::Continuation,
            Opcode::Text,
            Opcode::Binary,
            Opcode::Close,
            Opcode::Ping,
            Opcode::Pong,
        ] {
            assert_eq!(Opcode::from_u8(op.as_u8()), Some(op));
        }
        assert_eq!(Opcode::from_u8(0xFF), None);
    }

    #[test]
    fn test_opcode_classification() {
        assert!(Opcode::Close.is_control());
        assert!(Opcode::Ping.is_control());
        assert!(Opcode::Pong.is_control());
        assert!(!Opcode::Text.is_control());

        assert!(Opcode::Text.is_data());
        assert!(Opcode::Binary.is_data());
        assert!(!Opcode::Close.is_data());
    }

    #[test]
    fn test_close_code_roundtrip() {
        for code in [
            CloseCode::Normal,
            CloseCode::GoingAway,
            CloseCode::ProtocolError,
            CloseCode::MessageTooBig,
            CloseCode::InternalError,
        ] {
            assert_eq!(CloseCode::from_u16(code.as_u16()), Some(code));
        }
        assert_eq!(CloseCode::from_u16(9999), None);
    }

    #[test]
    fn test_text_frame() {
        let frame = Frame::text("hello");
        assert!(frame.fin);
        assert_eq!(frame.opcode, Opcode::Text);
        assert_eq!(frame.payload, b"hello");
        assert!(!frame.masked);
    }

    #[test]
    fn test_binary_frame() {
        let data = vec![1, 2, 3, 4];
        let frame = Frame::binary(data.clone());
        assert_eq!(frame.opcode, Opcode::Binary);
        assert_eq!(frame.payload, data);
    }

    #[test]
    fn test_close_frame() {
        let frame = Frame::close(CloseCode::Normal, "goodbye");
        assert_eq!(frame.opcode, Opcode::Close);
        // First two bytes are the close code (1000 = 0x03E8)
        assert_eq!(frame.payload[0], 0x03);
        assert_eq!(frame.payload[1], 0xE8);
        assert_eq!(&frame.payload[2..], b"goodbye");
    }

    #[test]
    fn test_ping_pong() {
        let ping = Frame::ping(b"heartbeat");
        assert_eq!(ping.opcode, Opcode::Ping);
        assert_eq!(ping.payload, b"heartbeat");

        let pong = Frame::pong(&ping.payload);
        assert_eq!(pong.opcode, Opcode::Pong);
        assert_eq!(pong.payload, ping.payload);
    }

    #[test]
    fn test_masking() {
        let mut frame = Frame::text("test");
        let original = frame.payload.clone();
        frame.set_mask([0xAA, 0xBB, 0xCC, 0xDD]);
        assert!(frame.masked);
        assert_ne!(frame.payload, original);

        // Re-apply mask to unmask
        frame.apply_mask();
        assert_eq!(frame.payload, original);
    }

    #[test]
    fn test_frame_serialization() {
        let frame = Frame::text("hi");
        let bytes = frame.to_bytes();
        // First byte: FIN=1 + opcode=1 => 0x81
        assert_eq!(bytes[0], 0x81);
        // Second byte: length=2
        assert_eq!(bytes[1], 2);
        assert_eq!(&bytes[2..], b"hi");
    }

    #[test]
    fn test_fragment_assembly() {
        let mut asm = FragmentAssembler::new();

        // First fragment
        let f1 = Frame {
            fin: false,
            opcode: Opcode::Text,
            payload: b"hel".to_vec(),
            masked: false,
            mask_key: None,
        };
        assert!(asm.add(f1).is_none());
        assert!(asm.is_assembling());

        // Continuation
        let f2 = Frame {
            fin: false,
            opcode: Opcode::Continuation,
            payload: b"lo ".to_vec(),
            masked: false,
            mask_key: None,
        };
        assert!(asm.add(f2).is_none());

        // Final fragment
        let f3 = Frame {
            fin: true,
            opcode: Opcode::Continuation,
            payload: b"world".to_vec(),
            masked: false,
            mask_key: None,
        };
        let result = asm.add(f3);
        assert!(result.is_some());
        let (opcode, payload) = result.unwrap();
        assert_eq!(opcode, Opcode::Text);
        assert_eq!(payload, b"hello world");
        assert!(!asm.is_assembling());
    }

    #[test]
    fn test_fragment_reset() {
        let mut asm = FragmentAssembler::new();
        let f1 = Frame {
            fin: false,
            opcode: Opcode::Binary,
            payload: vec![1, 2],
            masked: false,
            mask_key: None,
        };
        asm.add(f1);
        assert!(asm.is_assembling());
        asm.reset();
        assert!(!asm.is_assembling());
    }
}
