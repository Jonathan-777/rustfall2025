/// This module handles:
/// - Reading and analyzing file contents
/// - Calculating word count, line count, character frequency
/// - Getting file metadata (size)
/// - Error handling for file operations

use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::Instant;
use crate::models::{FileAnalysis, FileStats};
use crate::error::{ProcessingError, ProcessingResult};

/// Analyzer for file statistics and content
pub struct FileAnalyzer;

/// Validate that a file path is safe and accessible
fn validate_file_path(path: &Path) -> ProcessingResult<()> {
    // Check for invalid paths
    if path.as_os_str().is_empty() {
        return Err(ProcessingError::InvalidPath(
            "Empty file path".to_string()
        ));
    }
    
    // Check if path exists, with graceful handling for symlinks and special files
    match path.try_exists() {
        Ok(true) => {
            // File exists, validate it's not a problematic type
            match fs::metadata(path) {
                Ok(metadata) => {
                    // Reject directories
                    if metadata.is_dir() {
                        return Err(ProcessingError::DirectoryError(
                            format!("Path is a directory, not a file: {}",
                                path.display())
                        ));
                    }
                    // Accept regular files and symlinks to files
                    Ok(())
                }
                Err(e) => {
                    // Can't access metadata (permission issue, broken symlink, etc.)
                    Err(ProcessingError::from(e))
                }
            }
        }
        Ok(false) => {
            // File doesn't exist
            Err(ProcessingError::FileNotFound(
                path.to_string_lossy().to_string()
            ))
        }
        Err(e) => {
            // Can't determine if file exists (permission issue, etc.)
            Err(ProcessingError::from(e))
        }
    }
}

impl FileAnalyzer {
/// Analyze a single file and return comprehensive statistics
pub fn analyze_file(file_path: &str) -> ProcessingResult<FileAnalysis> {
    let start = Instant::now();
    let mut analysis = FileAnalysis::new(file_path.to_string());
    
    let path = std::path::Path::new(file_path);
    
    // Validate path is accessible and safe
    match validate_file_path(path) {
        Ok(_) => {}
        Err(e) => {
            analysis.add_error(e);
            analysis.processing_time = start.elapsed();
            return Ok(analysis);
        }
    }
    
    // Get file size with detailed error handling
    match fs::metadata(path) {
        Ok(metadata) => {
            // Check if it's actually a file
            if !metadata.is_file() {
                analysis.add_error(ProcessingError::DirectoryError(
                    format!("Path is not a regular file: {}", file_path)
                ));
                analysis.processing_time = start.elapsed();
                return Ok(analysis);
            }
            
            analysis.stats.size_bytes = metadata.len();
        }
        Err(e) => {
            analysis.add_error(ProcessingError::from(e));
            analysis.processing_time = start.elapsed();
            return Ok(analysis);
        }
    }
    
    // Open and read file with recovery strategy
    match File::open(path) {
        Ok(file) => {
            match Self::calculate_stats(file, &mut analysis.stats) {
                Ok(_) => {}
                Err(e) => {
                    analysis.add_error(e);
                }
            }
        }
        Err(e) => {
            analysis.add_error(ProcessingError::from(e));
        }
    }
    
    analysis.processing_time = start.elapsed();
    Ok(analysis)
}    /// Calculate statistics for a file with robust error recovery
    fn calculate_stats(file: File, stats: &mut FileStats) -> ProcessingResult<()> {
        let reader = BufReader::new(file);
        let mut error_count = 0;
        const MAX_LINE_ERRORS: usize = 10; // Allow some read errors before aborting
        
        for (line_num, line_result) in reader.lines().enumerate() {
            match line_result {
                Ok(line_content) => {
                    stats.line_count += 1;
                    error_count = 0; // Reset error counter on success
                    
                    // Count words (split by whitespace)
                    let words: Vec<&str> = line_content.split_whitespace().collect();
                    stats.word_count += words.len();
                    
                    // Count character frequencies (only letters, case-insensitive)
                    for ch in line_content.chars() {
                        let lower = ch.to_lowercase().to_string();
                        // Only count alphabetic characters for frequency
                        if ch.is_alphabetic() {
                            for c in lower.chars() {
                                *stats.char_frequencies.entry(c).or_insert(0) += 1;
                            }
                        } else {
                            // Count whitespace and punctuation as single entries
                            *stats.char_frequencies.entry(ch).or_insert(0) += 1;
                        }
                    }
                }
                Err(e) => {
                    error_count += 1;
                    
                    // Log warning but continue processing (graceful degradation)
                    if error_count <= 3 {
                        eprintln!(
                            "Warning: Failed to read line {}: {} (continuing...)",
                            line_num,
                            e
                        );
                    }
                    
                    // Abort if too many consecutive read errors
                    if error_count > MAX_LINE_ERRORS {
                        return Err(ProcessingError::CorruptedFile(
                            format!(
                                "Too many read errors ({}+ errors, file may be corrupted)",
                                MAX_LINE_ERRORS
                            )
                        ));
                    }
                }
            }
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_analyze_nonexistent_file() {
        let result = FileAnalyzer::analyze_file("nonexistent_file.txt").unwrap();
        assert!(!result.is_successful());
        assert!(!result.errors.is_empty());
    }
}
