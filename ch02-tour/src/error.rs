use std::fmt;
use std::io;

#[derive(Debug)]
pub enum AppError {
    /// Invalid command-line arguments
    /// 🎓 C#: throw new ArgumentException(message);
    /// Rust: return Err(AppError::InvalidArgs(message));
    InvalidArgs(String),

    /// File read error (wraps io::Error)
    /// 🎓 C#: throw new IOException("...", innerException);
    /// Rust: return Err(AppError::FileRead(io_error));
    FileRead(io::Error),

    /// Search-related error
    /// 🎓 C#: throw new ApplicationException(message);
    /// Rust: return Err(AppError::SearchError(message));
    SearchError(String),
}


impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Rust: Pattern match on enum variants
        match self {
            // Destructure the variant and extract the message
            AppError::InvalidArgs(msg) => write!(f, "Invalid arguments: {}", msg),

            // Destructure and extract the io::Error, which has its own Display
            AppError::FileRead(err) => write!(f, "File read error: {}", err),

            // Custom search error message
            AppError::SearchError(msg) => write!(f, "Search error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {
}
impl From<io::Error> for AppError {
    fn from(err: io::Error) -> AppError {
        AppError::FileRead(err)
    }
}

impl From<&str> for AppError {
    fn from(msg: &str) -> AppError {
        AppError::InvalidArgs(msg.to_string())
    }
}
