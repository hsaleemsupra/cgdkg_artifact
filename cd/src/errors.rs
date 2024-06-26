use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct InvalidArgumentError {
    pub message: String,
}

impl fmt::Display for InvalidArgumentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Semantic error in argument: {}", &self.message)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MalformedPublicKeyError {
    pub key_bytes: Option<Vec<u8>>,
    pub internal_error: String,
}

impl fmt::Display for MalformedPublicKeyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Malformed public key: 0x{:?}. Internal error: {}",
            &self.key_bytes.as_ref().map(hex::encode),
            &self.internal_error
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InternalError {
    pub internal_error: String,
}
