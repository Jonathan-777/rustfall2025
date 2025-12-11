/// Core library module for the Parallel File Processor
/// 
/// This module provides:
/// - Custom thread pool implementation using only std::thread and std::sync
/// - File analysis and statistics collection
/// - Progress tracking
/// - Error handling
/// - Task distribution

pub mod models;
pub mod thread_pool;
pub mod file_processor;
pub mod progress_tracker;
pub mod error;
pub mod downloader;
pub mod logger;
pub mod main_helper;

pub use models::{FileAnalysis, FileStats};
pub use thread_pool::ThreadPool;
pub use file_processor::FileAnalyzer;
pub use progress_tracker::{ProgressTracker, SharedProgressTracker};
pub use error::{ProcessingError, ProcessingResult};
pub use downloader::{download_books_to_meet_demand, get_available_books};
