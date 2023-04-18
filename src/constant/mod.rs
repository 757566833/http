use std::fmt::Display;

// 回车
pub const CR: &[u8; 1] = b"\r";

pub const DIVER: &[u8; 4] = b"\r\n\r\n";

pub enum StatusCode {
    OK = 200,
    BadRequest = 400,
    NotFound = 404,
}
impl Display for StatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StatusCode::OK => write!(f, "200 OK"),
            StatusCode::BadRequest => write!(f, "400 Bad Request"),
            StatusCode::NotFound => write!(f, "404 Not Found"),
        }
    }
}