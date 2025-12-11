/// - Shared progress state across threads
/// - Thread-safe updates to processing statistics
/// - Progress reporting
/// - Cancellation signals

use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;
use crate::models::FileAnalysis;

/// Tracks overall progress of file processing
#[derive(Debug)]
pub struct ProgressTracker {
    /// All completed file analyses
    completed_analyses: Vec<FileAnalysis>,
    
    /// Total files discovered for processing
    total_files: usize,
    
    /// Number of files completed
    files_completed: usize,
    
    /// Total errors encountered
    total_errors: usize,
    
    /// When processing started
    start_time: Instant,
    
    /// List of errors with context
    errors_log: Vec<String>,
}

impl ProgressTracker {
    /// Create a new progress tracker
    pub fn new() -> Self {
        ProgressTracker {
            completed_analyses: Vec::new(),
            total_files: 0,
            files_completed: 0,
            total_errors: 0,
            start_time: Instant::now(),
            errors_log: Vec::new(),
        }
    }
    
    /// Set the total number of files to process
    pub fn set_total_files(&mut self, total: usize) {
        self.total_files = total;
    }
    
    /// Record a completed file analysis
    pub fn record_completion(&mut self, analysis: FileAnalysis) {
        self.files_completed += 1;
        self.total_errors += analysis.errors.len();
        
        for error in &analysis.errors {
            self.errors_log.push(format!(
                "{}: {}",
                analysis.filename,
                error
            ));
        }
        
        self.completed_analyses.push(analysis);
    }
    
    /// Get current progress percentage
    pub fn progress_percentage(&self) -> f64 {
        if self.total_files == 0 {
            0.0
        } else {
            (self.files_completed as f64 / self.total_files as f64) * 100.0
        }
    }
    
    /// Get elapsed time since processing started
    pub fn elapsed_time(&self) -> std::time::Duration {
        self.start_time.elapsed()
    }
    
    /// Get statistics summary
    pub fn get_summary(&self) -> ProgressSummary {
        ProgressSummary {
            total_files: self.total_files,
            files_completed: self.files_completed,
            total_errors: self.total_errors,
            elapsed_time: self.elapsed_time(),
            progress_percentage: self.progress_percentage(),
        }
    }
    
    /// Get all completed analyses
    pub fn get_completed_analyses(&self) -> &[FileAnalysis] {
        &self.completed_analyses
    }
    
    /// Get error log
    pub fn get_errors(&self) -> &[String] {
        &self.errors_log
    }
}

impl Default for ProgressTracker {
    fn default() -> Self {
        Self::new()
    }
}

/// Summary statistics of progress
#[derive(Debug, Clone)]
pub struct ProgressSummary {
    pub total_files: usize,
    pub files_completed: usize,
    pub total_errors: usize,
    pub elapsed_time: std::time::Duration,
    pub progress_percentage: f64,
}

impl ProgressSummary {
    /// Format summary as a human-readable string
    pub fn to_display_string(&self) -> String {
        format!(
            "Progress: {}/{} files ({:.1}%) | Errors: {} | Elapsed: {:.2}s",
            self.files_completed,
            self.total_files,
            self.progress_percentage,
            self.total_errors,
            self.elapsed_time.as_secs_f64()
        )
    }
}

/// Thread-safe wrapper for ProgressTracker
pub struct SharedProgressTracker {
    inner: Arc<Mutex<ProgressTracker>>,
    cancel_flag: Arc<AtomicBool>,
}

impl SharedProgressTracker {
    /// Create a new shared progress tracker
    pub fn new() -> Self {
        SharedProgressTracker {
            inner: Arc::new(Mutex::new(ProgressTracker::new())),
            cancel_flag: Arc::new(AtomicBool::new(false)),
        }
    }
    
    /// Set total files (thread-safe)
    pub fn set_total_files(&self, total: usize) {
        if let Ok(mut tracker) = self.inner.lock() {
            tracker.set_total_files(total);
        }
    }
    
    /// Record a completion (thread-safe)
    pub fn record_completion(&self, analysis: FileAnalysis) {
        if let Ok(mut tracker) = self.inner.lock() {
            tracker.record_completion(analysis);
        }
    }
    
    /// Get current summary (thread-safe)
    pub fn get_summary(&self) -> Option<ProgressSummary> {
        self.inner.lock().ok().map(|tracker| tracker.get_summary())
    }
    
    /// Get completed analyses (thread-safe)
    pub fn get_completed_analyses(&self) -> Option<Vec<FileAnalysis>> {
        self.inner.lock().ok().map(|tracker| tracker.get_completed_analyses().to_vec())
    }
    
    /// Get error log (thread-safe)
    pub fn get_errors(&self) -> Option<Vec<String>> {
        self.inner.lock().ok().map(|tracker| tracker.get_errors().to_vec())
    }
    
    /// Check if cancellation was requested
    pub fn is_cancelled(&self) -> bool {
        self.cancel_flag.load(Ordering::Relaxed)
    }
    
    /// Request cancellation
    pub fn cancel(&self) {
        self.cancel_flag.store(true, Ordering::Relaxed);
    }
    
    /// Clone the inner Arc for use by workers
    pub fn clone_inner(&self) -> Arc<Mutex<ProgressTracker>> {
        Arc::clone(&self.inner)
    }
    
    /// Clone the cancel flag for use by workers
    pub fn clone_cancel_flag(&self) -> Arc<AtomicBool> {
        Arc::clone(&self.cancel_flag)
    }
}

impl Default for SharedProgressTracker {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for SharedProgressTracker {
    fn clone(&self) -> Self {
        SharedProgressTracker {
            inner: Arc::clone(&self.inner),
            cancel_flag: Arc::clone(&self.cancel_flag),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_progress_tracker_creation() {
        let tracker = ProgressTracker::new();
        assert_eq!(tracker.files_completed, 0);
        assert_eq!(tracker.total_files, 0);
        assert_eq!(tracker.total_errors, 0);
    }
    
    #[test]
    fn test_progress_percentage() {
        let mut tracker = ProgressTracker::new();
        tracker.set_total_files(10);
        assert_eq!(tracker.progress_percentage(), 0.0);
        
        tracker.files_completed = 5;
        assert_eq!(tracker.progress_percentage(), 50.0);
        
        tracker.files_completed = 10;
        assert_eq!(tracker.progress_percentage(), 100.0);
    }
    
    #[test]
    fn test_shared_progress_tracker() {
        let tracker = SharedProgressTracker::new();
        tracker.set_total_files(5);
        
        let analysis = FileAnalysis::new("test.txt".to_string());
        tracker.record_completion(analysis);
        
        let summary = tracker.get_summary().unwrap();
        assert_eq!(summary.total_files, 5);
        assert_eq!(summary.files_completed, 1);
    }
}
