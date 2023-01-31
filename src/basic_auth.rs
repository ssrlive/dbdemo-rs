use base64::{engine::general_purpose, Engine as _};

// Basic authentication, for example:
// Basic base64(username:password)
#[derive(Debug, Clone)]
pub struct BasicAuth {
    pub username: String,
    pub password: String,
}

impl BasicAuth {
    pub fn new(username: String, password: String) -> Self {
        Self { username, password }
    }

    pub fn from_header(header: &str) -> Option<Self> {
        let header = header.trim_start_matches("Basic ");
        let decoded = general_purpose::STANDARD.decode(header).ok()?;
        let decoded = String::from_utf8(decoded).ok()?;
        let mut split = decoded.splitn(2, ':');
        let username = split.next()?.to_string();
        let password = split.next()?.to_string();
        Some(Self::new(username, password))
    }
}
