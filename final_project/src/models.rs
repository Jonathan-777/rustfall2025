use std::collections::HashMap;
use std::time::Duration;
use crate::error::ProcessingError;

/// Statistics collected during file analysis
#[derive(Debug, Clone)]
pub struct FileStats {
    /// Total number of words in the file
    pub word_count: usize,
    
    /// Total number of lines in the file
    pub line_count: usize,
    
    /// Character frequency distribution
    pub char_frequencies: HashMap<char, usize>,
    
    /// File size in bytes
    pub size_bytes: u64,
}

impl FileStats {
    pub fn new() -> Self {
        FileStats {
            word_count: 0,
            line_count: 0,
            char_frequencies: HashMap::new(),
            size_bytes: 0,
        }
    }
}

impl Default for FileStats {
    fn default() -> Self {
        Self::new()
    }
}

/// Complete analysis result for a single file
#[derive(Debug, Clone)]
pub struct FileAnalysis {
    pub filename: String,
    
    pub stats: FileStats,
    
    pub errors: Vec<ProcessingError>,
    
    pub processing_time: Duration,
}

impl FileAnalysis {
    /// Create a new FileAnalysis with default values
    pub fn new(filename: String) -> Self {
        FileAnalysis {
            filename,
            stats: FileStats::new(),
            errors: Vec::new(),
            processing_time: Duration::ZERO,
        }
    }
    
    /// Check if the analysis completed successfully (no errors)
    pub fn is_successful(&self) -> bool {
        self.errors.is_empty()
    }
    
    /// Add an error to the analysis
    pub fn add_error(&mut self, error: ProcessingError) {
        self.errors.push(error);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_file_stats_new() {
        let stats = FileStats::new();
        assert_eq!(stats.word_count, 0);
        assert_eq!(stats.line_count, 0);
        assert_eq!(stats.size_bytes, 0);
        assert!(stats.char_frequencies.is_empty());
    }
    
    #[test]
    fn test_file_analysis_new() {
        let analysis = FileAnalysis::new("test.txt".to_string());
        assert_eq!(analysis.filename, "test.txt");
        assert!(analysis.is_successful());
        assert!(analysis.errors.is_empty());
    }
}
