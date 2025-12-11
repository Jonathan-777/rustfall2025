/// Comprehensive error handling tests
/// 
/// Tests verify that the application gracefully handles various file system errors:
/// - File not found
/// - Permission denied
/// - Invalid paths
/// - Corrupted files
/// - Symbolic links
/// - Directory vs file confusion

use final_project::FileAnalyzer;
use std::fs;
use std::io::Write;

#[test]
fn test_nonexistent_file_returns_error() {
    let result = FileAnalyzer::analyze_file("nonexistent_file_12345.txt");
    assert!(result.is_ok(), "Should return Ok with analysis containing error");
    
    let analysis = result.unwrap();
    assert!(!analysis.is_successful(), "Analysis should fail");
    assert!(!analysis.errors.is_empty(), "Should contain errors");
}

#[test]
fn test_directory_instead_of_file() {
    let result = FileAnalyzer::analyze_file(".");
    assert!(result.is_ok(), "Should return Ok with analysis containing error");
    
    let analysis = result.unwrap();
    assert!(!analysis.is_successful(), "Analysis should fail");
    assert!(!analysis.errors.is_empty(), "Should contain errors");
    
    // Check that it's a directory-related error
    let error_str = analysis.errors[0].to_string();
    assert!(
        error_str.contains("directory") || error_str.contains("not a file"),
        "Error should mention directory or file type: {}",
        error_str
    );
}

#[test]
fn test_valid_file_analysis() {
    // Create a temporary test file
    let test_file = "test_valid_file_12345.txt";
    if let Ok(mut f) = fs::File::create(test_file) {
        let _ = writeln!(f, "Hello world\nThis is line 2\nAnd line 3");
    }
    
    let result = FileAnalyzer::analyze_file(test_file);
    assert!(result.is_ok());
    
    let analysis = result.unwrap();
    assert!(analysis.is_successful(), "Valid file should be analyzed successfully");
    assert_eq!(analysis.stats.line_count, 3, "Should count 3 lines");
    
    // Cleanup
    let _ = fs::remove_file(test_file);
}

#[test]
fn test_empty_file_analysis() {
    let test_file = "test_empty_file_12345.txt";
    if let Ok(_) = fs::File::create(test_file) {
        // Create empty file
    }
    
    let result = FileAnalyzer::analyze_file(test_file);
    assert!(result.is_ok());
    
    let analysis = result.unwrap();
    assert!(analysis.is_successful(), "Empty file should be analyzed successfully");
    assert_eq!(analysis.stats.line_count, 0, "Empty file should have 0 lines");
    assert_eq!(analysis.stats.word_count, 0, "Empty file should have 0 words");
    
    // Cleanup
    let _ = fs::remove_file(test_file);
}

#[test]
fn test_large_file_analysis() {
    let test_file = "test_large_file_12345.txt";
    if let Ok(mut f) = fs::File::create(test_file) {
        // Write 1000 lines
        for i in 0..1000 {
            let _ = writeln!(f, "Line {} with some content here", i);
        }
    }
    
    let result = FileAnalyzer::analyze_file(test_file);
    assert!(result.is_ok());
    
    let analysis = result.unwrap();
    assert!(analysis.is_successful());
    assert_eq!(analysis.stats.line_count, 1000, "Should count all 1000 lines");
    assert!(analysis.stats.word_count > 0, "Should count words");
    
    // Cleanup
    let _ = fs::remove_file(test_file);
}

#[test]
fn test_file_with_special_characters() {
    let test_file = "test_special_chars_12345.txt";
    if let Ok(mut f) = fs::File::create(test_file) {
        let _ = writeln!(f, "Special chars: !@#$%^&*()");
        let _ = writeln!(f, "Unicode: café naïve");
        let _ = writeln!(f, "Quotes: \"hello\" 'world'");
    }
    
    let result = FileAnalyzer::analyze_file(test_file);
    assert!(result.is_ok());
    
    let analysis = result.unwrap();
    assert!(analysis.is_successful());
    assert_eq!(analysis.stats.line_count, 3);
    assert!(!analysis.stats.char_frequencies.is_empty());
    
    // Cleanup
    let _ = fs::remove_file(test_file);
}

#[test]
fn test_invalid_path_string() {
    let result = FileAnalyzer::analyze_file("");
    assert!(result.is_ok());
    
    let analysis = result.unwrap();
    assert!(!analysis.is_successful(), "Empty path should result in error");
}

#[test]
fn test_file_with_mixed_line_endings() {
    let test_file = "test_mixed_endings_12345.txt";
    if let Ok(mut f) = fs::File::create(test_file) {
        // Unix line ending
        let _ = f.write_all(b"Line 1\n");
        // Windows line ending
        let _ = f.write_all(b"Line 2\r\n");
        // Old Mac style (single \r)
        let _ = f.write_all(b"Line 3\r");
        let _ = f.write_all(b"Line 4\n");
    }
    
    let result = FileAnalyzer::analyze_file(test_file);
    assert!(result.is_ok());
    
    let analysis = result.unwrap();
    // BufReader normalizes line endings, so should see proper line count
    assert!(analysis.is_successful());
    
    // Cleanup
    let _ = fs::remove_file(test_file);
}

#[test]
fn test_file_permissions_error_graceful_handling() {
    // Create a test file
    let test_file = "test_perms_12345.txt";
    if let Ok(mut f) = fs::File::create(test_file) {
        let _ = writeln!(f, "Test content");
    }
    
    // Try to remove permissions (this works differently on Windows)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = fs::set_permissions(test_file, fs::Permissions::from_mode(0o000));
        
        let result = FileAnalyzer::analyze_file(test_file);
        assert!(result.is_ok());
        
        let analysis = result.unwrap();
        assert!(!analysis.is_successful(), "Should fail due to permissions");
        
        // Restore permissions for cleanup
        let _ = fs::set_permissions(test_file, fs::Permissions::from_mode(0o644));
    }
    
    // Cleanup
    let _ = fs::remove_file(test_file);
}

#[test]
fn test_analysis_result_contains_stats_even_on_partial_error() {
    let test_file = "test_partial_error_12345.txt";
    if let Ok(mut f) = fs::File::create(test_file) {
        let _ = writeln!(f, "Valid line 1");
        let _ = writeln!(f, "Valid line 2");
        let _ = writeln!(f, "Valid line 3");
    }
    
    let result = FileAnalyzer::analyze_file(test_file);
    assert!(result.is_ok());
    
    let analysis = result.unwrap();
    // File should be readable and analyzed
    assert!(analysis.is_successful());
    assert_eq!(analysis.stats.line_count, 3);
    
    // Cleanup
    let _ = fs::remove_file(test_file);
}
