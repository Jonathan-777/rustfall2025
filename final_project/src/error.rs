use std::fmt;
use std::io;

/// Represents different types of processing errors that can occur
#[derive(Debug, Clone)]
pub enum ProcessingError {
    /// File I/O error with context
    IoError(String),
    
    /// File not found error
    FileNotFound(String),
    
    /// Permission denied when accessing file
    PermissionDenied(String),
    
    /// Error parsing file content
    ParseError(String),
    
    /// Directory traversal error
    DirectoryError(String),
    
    /// Timeout or cancellation error
    Cancelled(String),
    
    /// Invalid path or file name
    InvalidPath(String),
    
    /// File is corrupted or unreadable
    CorruptedFile(String),
    
    /// Insufficient disk space or other OS resources
    SystemResource(String),
    
    /// Symbolic link issues
    SymlinkError(String),
}

impl fmt::Display for ProcessingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProcessingError::IoError(msg) => write!(f, "IO Error: {}", msg),
            ProcessingError::FileNotFound(path) => write!(f, "File not found: {}", path),
            ProcessingError::PermissionDenied(path) => write!(f, "Permission denied: {}", path),
            ProcessingError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            ProcessingError::DirectoryError(msg) => write!(f, "Directory error: {}", msg),
            ProcessingError::Cancelled(msg) => write!(f, "Cancelled: {}", msg),
            ProcessingError::InvalidPath(path) => write!(f, "Invalid path: {}", path),
            ProcessingError::CorruptedFile(path) => write!(f, "Corrupted or unreadable file: {}", path),
            ProcessingError::SystemResource(msg) => write!(f, "System resource error: {}", msg),
            ProcessingError::SymlinkError(msg) => write!(f, "Symbolic link error: {}", msg),
        }
    }
}

impl From<io::Error> for ProcessingError {
    fn from(err: io::Error) -> Self {
        match err.kind() {
            io::ErrorKind::NotFound => ProcessingError::FileNotFound(err.to_string()),
            io::ErrorKind::PermissionDenied => ProcessingError::PermissionDenied(err.to_string()),
            io::ErrorKind::InvalidFilename | io::ErrorKind::InvalidInput => {
                ProcessingError::InvalidPath(err.to_string())
            }
            io::ErrorKind::InvalidData => ProcessingError::CorruptedFile(err.to_string()),
            io::ErrorKind::StorageFull | io::ErrorKind::OutOfMemory => {
                ProcessingError::SystemResource(err.to_string())
            }
            io::ErrorKind::NotADirectory | io::ErrorKind::IsADirectory => {
                ProcessingError::DirectoryError(err.to_string())
            }
            _ => ProcessingError::IoError(err.to_string()),
        }
    }
}

pub type ProcessingResult<T> = Result<T, ProcessingError>;
