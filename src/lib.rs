trait FromBytes: Sized {
    fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn std::error::Error>>;
}

trait ToBytes {
    fn to_bytes(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>>;
}
