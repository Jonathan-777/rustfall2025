/// Integration tests for the Parallel File Processor
use final_project::{FileAnalyzer, ThreadPool, SharedProgressTracker, FileAnalysis};
use std::fs::File;
use std::io::Write;
use std::thread;
use std::time::Duration;

/// Create a temporary test file with known content
fn create_test_file(path: &str, content: &str) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

/// Clean up test files
fn cleanup_test_file(path: &str) {
    let _ = std::fs::remove_file(path);
}

#[test]
fn test_simple_file_analysis() {
    let test_file = "test_simple.txt";
    let content = "Hello world\nThis is a test";
    
    create_test_file(test_file, content).expect("Failed to create test file");
    
    let result = FileAnalyzer::analyze_file(test_file).expect("Analysis failed");
    
    assert!(result.is_successful(), "Analysis should succeed");
    assert_eq!(result.stats.line_count, 2, "Should have 2 lines");
    assert_eq!(result.stats.word_count, 6, "Should have 6 words");
    assert!(result.stats.size_bytes > 0, "File size should be > 0");
    assert!(!result.stats.char_frequencies.is_empty(), "Should have character frequencies");
    
    cleanup_test_file(test_file);
}

#[test]
fn test_empty_file_analysis() {
    let test_file = "test_empty.txt";
    
    create_test_file(test_file, "").expect("Failed to create test file");
    
    let result = FileAnalyzer::analyze_file(test_file).expect("Analysis failed");
    
    assert!(result.is_successful(), "Analysis should succeed for empty file");
    assert_eq!(result.stats.line_count, 0, "Empty file should have 0 lines");
    assert_eq!(result.stats.word_count, 0, "Empty file should have 0 words");
    assert_eq!(result.stats.size_bytes, 0, "Empty file should have size 0");
    
    cleanup_test_file(test_file);
}

#[test]
fn test_large_file_analysis() {
    let test_file = "test_large.txt";
    let mut content = String::new();
    
    // Create a file with 1000 lines
    for i in 0..1000 {
        content.push_str(&format!("Line {} with some text content\n", i));
    }
    
    create_test_file(test_file, &content).expect("Failed to create test file");
    
    let result = FileAnalyzer::analyze_file(test_file).expect("Analysis failed");
    
    assert!(result.is_successful(), "Analysis should succeed");
    assert_eq!(result.stats.line_count, 1000, "Should have 1000 lines");
    assert!(result.stats.word_count > 1000, "Should have at least 1000 words");
    
    cleanup_test_file(test_file);
}

#[test]
fn test_character_frequency() {
    let test_file = "test_freq.txt";
    let content = "aaa bbb ccc aaa";
    
    create_test_file(test_file, content).expect("Failed to create test file");
    
    let result = FileAnalyzer::analyze_file(test_file).expect("Analysis failed");
    
    let freq = &result.stats.char_frequencies;
    assert!(freq.get(&'a').is_some(), "Should have 'a' in frequency map");
    assert!(freq.get(&'b').is_some(), "Should have 'b' in frequency map");
    
    cleanup_test_file(test_file);
}

#[test]
fn test_nonexistent_file() {
    let result = FileAnalyzer::analyze_file("nonexistent_file_xyz.txt")
        .expect("Should return Ok with error");
    
    assert!(!result.is_successful(), "Analysis should fail for nonexistent file");
    assert!(!result.errors.is_empty(), "Should have errors");
}

#[test]
fn test_thread_pool_creation() {
    let pool = ThreadPool::new(4);
    assert_eq!(pool.num_workers(), 4);
}

#[test]
fn test_thread_pool_task_execution() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    
    let pool = ThreadPool::new(4);
    let counter = Arc::new(AtomicUsize::new(0));
    
    for _ in 0..10 {
        let c = Arc::clone(&counter);
        pool.execute(move || {
            c.fetch_add(1, Ordering::SeqCst);
        });
    }
    
    // Give threads time to execute
    thread::sleep(Duration::from_millis(100));
    
    assert_eq!(counter.load(Ordering::SeqCst), 10, "All tasks should execute");
}

#[test]
fn test_progress_tracker_updates() {
    let tracker = SharedProgressTracker::new();
    tracker.set_total_files(5);
    
    // Record some completions
    for i in 0..5 {
        let analysis = FileAnalysis::new(format!("file_{}.txt", i));
        tracker.record_completion(analysis);
    }
    
    let summary = tracker.get_summary().expect("Should get summary");
    assert_eq!(summary.total_files, 5);
    assert_eq!(summary.files_completed, 5);
    assert_eq!(summary.progress_percentage, 100.0);
}

#[test]
fn test_progress_tracker_cancellation() {
    let tracker = SharedProgressTracker::new();
    
    assert!(!tracker.is_cancelled(), "Should not be cancelled initially");
    
    tracker.cancel();
    
    assert!(tracker.is_cancelled(), "Should be cancelled after cancel()");
}

#[test]
fn test_concurrent_file_analysis() {
    let test_files = vec![
        ("test_concurrent_1.txt", "File one content\n"),
        ("test_concurrent_2.txt", "File two content here\n"),
        ("test_concurrent_3.txt", "File three has more content\n"),
    ];
    
    // Create test files
    for (path, content) in &test_files {
        create_test_file(path, content).expect("Failed to create test file");
    }
    
    let pool = ThreadPool::new(3);
    let tracker = SharedProgressTracker::new();
    tracker.set_total_files(test_files.len());
    
    // Submit analysis tasks
    for (path, _) in &test_files {
        let path = path.to_string();
        let tracker_inner = tracker.clone();
        
        pool.execute(move || {
            if let Ok(analysis) = final_project::FileAnalyzer::analyze_file(&path) {
                tracker_inner.record_completion(analysis);
            }
        });
    }
    
    // Give threads time to execute
    thread::sleep(Duration::from_millis(200));
    
    let summary = tracker.get_summary().expect("Should get summary");
    assert_eq!(summary.files_completed, 3, "All files should be processed");
    
    // Clean up
    for (path, _) in &test_files {
        cleanup_test_file(path);
    }
}

#[test]
fn test_file_stats_ordering() {
    let test_file = "test_ordering.txt";
    let content = "The quick brown fox jumps over the lazy dog\n\
                   The fox is quick\n\
                   Dogs are lazy";
    
    create_test_file(test_file, content).expect("Failed to create test file");
    
    let result = FileAnalyzer::analyze_file(test_file).expect("Analysis failed");
    
    // Find character with highest frequency
    let max_char = result.stats.char_frequencies
        .iter()
        .max_by_key(|&(_, count)| count)
        .map(|(&c, count)| (c, count));
    
    assert!(max_char.is_some(), "Should have character frequencies");
    
    cleanup_test_file(test_file);
}

#[test]
fn test_multiple_concurrent_thread_pools() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    
    let pool1 = ThreadPool::new(2);
    let pool2 = ThreadPool::new(2);
    
    let counter1 = Arc::new(AtomicUsize::new(0));
    let counter2 = Arc::new(AtomicUsize::new(0));
    
    // Submit tasks to pool 1
    for _ in 0..5 {
        let c = Arc::clone(&counter1);
        pool1.execute(move || {
            c.fetch_add(1, Ordering::SeqCst);
        });
    }
    
    // Submit tasks to pool 2
    for _ in 0..5 {
        let c = Arc::clone(&counter2);
        pool2.execute(move || {
            c.fetch_add(1, Ordering::SeqCst);
        });
    }
    
    thread::sleep(Duration::from_millis(100));
    
    assert_eq!(counter1.load(Ordering::SeqCst), 5);
    assert_eq!(counter2.load(Ordering::SeqCst), 5);
}
