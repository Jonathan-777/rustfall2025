use std::fs;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;
use std::io::{self, Write};

use crate::{
    FileAnalyzer, ThreadPool, SharedProgressTracker, FileAnalysis, 
    logger
};

/// Global cancellation flag - set to true when Ctrl+C is pressed
pub static CANCEL_REQUESTED: AtomicBool = AtomicBool::new(false);

pub fn start_ctrlc_listener() {
    thread::spawn(|| {
        if let Err(e) = ctrlc::set_handler({
            move || {
                CANCEL_REQUESTED.store(true, Ordering::SeqCst);
                println!("\n\n{:*<80}", "");
                println!(" CANCELLATION REQUESTED - Safely shutting down...");
                println!("{:*<80}\n", "");
                std::process::exit(0);
            }
        }) {
            eprintln!("Error setting up Ctrl+C handler: {}", e);
        }
    });
    // Give the handler thread time to register
    thread::sleep(Duration::from_millis(10));
}

pub struct ProcessorConfig {
    /// Number of worker threads
    pub num_workers: usize,
    /// Directories to scan for files
    pub directories: Vec<String>,
    /// File extensions to process
    pub extensions: Vec<String>,
}

impl ProcessorConfig {
    pub fn new(num_workers: usize) -> Self {
        ProcessorConfig {
            num_workers,
            directories: Vec::new(),
            extensions: vec![
                "txt".to_string(),
                "md".to_string(),
            ],
        }
    }
}

/// Prompt user to add directories for file processing
/// Handles existing directories, directory creation, and file type filtering
pub fn get_directories_from_user() -> Vec<String> {
    let mut directories = Vec::new();
    let default_dir = "./books".to_string();
    
    loop {
        if CANCEL_REQUESTED.load(Ordering::SeqCst) {
            return directories;
        }
        
        let prompt = if directories.is_empty() {
            format!("Enter directory path (the default is: {}, or add your own direcory. The default will download books from https://www.gutenberg.org if needed ), type 'done' to finish adding directories: ", default_dir)
        } else {
            format!("Enter another directory path, or 'done' to finish: ")
        };
        
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Error reading input");
            continue;
        }
        
        let trimmed = input.trim();
        
        // Check for cancellation
        if CANCEL_REQUESTED.load(Ordering::SeqCst) {
            return directories;
        }
        
        // Handle "done" command
        if trimmed.eq_ignore_ascii_case("done") {
            if directories.is_empty() {
                println!("Using default directory: {}", default_dir);
                directories.push(default_dir);
            }
            break;
        }
        
        // Handle empty input - use default
        let dir_path = if trimmed.is_empty() {
            if directories.is_empty() {
                println!("Using default directory: {}", default_dir);
                default_dir.clone()
            } else {
                println!("Please enter a valid directory path");
                continue;
            }
        } else {
            trimmed.to_string()
        };
        
        // Check if directory exists
        if fs::metadata(&dir_path).is_ok() {
            // Directory exists
            match fs::metadata(&dir_path) {
                Ok(metadata) if metadata.is_dir() => {
                    println!(" Directory '{}' found", dir_path);
                    
                    // Check for supported file types
                    match fs::read_dir(&dir_path) {
                        Ok(entries) => {
                            let has_supported_files = entries
                                .flatten()
                                .any(|entry| {
                                    if let Ok(metadata) = entry.metadata() {
                                        if metadata.is_file() {
                                            if let Some(ext) = entry.path().extension() {
                                                let ext_str = ext.to_string_lossy().to_lowercase();
                                                return ext_str == "txt" || ext_str == "md";
                                            }
                                        }
                                    }
                                    false
                                });
                            
                            if !has_supported_files {
                                println!("Warning: No .txt or .md files found in this directory");
                                println!("  (Other file types will be ignored)");
                            }
                            
                            if !directories.contains(&dir_path) {
                                directories.push(dir_path);
                            } else {
                                println!(" This directory is already added");
                            }
                        }
                        Err(e) => {
                            eprintln!(" Cannot read directory '{}': {}", dir_path, e);
                        }
                    }
                }
                Ok(_) => {
                    eprintln!(" Path '{}' exists but is not a directory", dir_path);
                }
                Err(e) => {
                    eprintln!(" Cannot access '{}': {}", dir_path, e);
                }
            }
        } else {
            // Directory doesn't exist - ask if user wants to create it
            println!(" Directory '{}' does not exist", dir_path);
            
            print!("Would you like to create this directory? (yes/no): ");
            io::stdout().flush().unwrap();
            
            let mut response = String::new();
            if io::stdin().read_line(&mut response).is_err() {
                eprintln!("Error reading input");
                continue;
            }
            
            if response.trim().eq_ignore_ascii_case("yes") || response.trim().eq_ignore_ascii_case("y") {
                match fs::create_dir_all(&dir_path) {
                    Ok(_) => {
                        println!(" Directory '{}' created successfully", dir_path);
                        directories.push(dir_path);
                    }
                    Err(e) => {
                        eprintln!(" Failed to create directory '{}': {}", dir_path, e);
                    }
                }
            } else {
                println!("Skipping directory '{}'", dir_path);
            }
        }
    }
    
    directories
}

/// Recursively discover files in directories matching specified extensions
/// Handles all potential file system errors gracefully
pub fn discover_files(directories: &[String], extensions: &[String]) -> Vec<String> {
    let mut files = Vec::new();
    
    for dir in directories {
        discover_files_in_dir(dir, extensions, &mut files);
    }
    
    files
}


/// Helper function to recursively discover files with error handling
fn discover_files_in_dir(dir: &str, extensions: &[String], files: &mut Vec<String>) {
    match fs::read_dir(dir) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let path = entry.path();
                        
                        // Handle symbolic links safely
                        match path.metadata() {
                            Ok(metadata) => {
                                if metadata.is_dir() {
                                    // Recursively scan subdirectories
                                    discover_files_in_dir(
                                        &path.to_string_lossy().to_string(),
                                        extensions,
                                    files,
                                    );
                                } else if metadata.is_file() {
                                    // Check if file has matching extension
                                    if let Some(ext) = path.extension() {
                                        let ext_str = ext.to_string_lossy().to_string();
                                        if extensions.contains(&ext_str) {
                                            files.push(path.to_string_lossy().to_string());
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                // Warn about inaccessible files but continue
                                eprintln!(
                                    "Warning: Cannot access {}: {} (skipping)",
                                    path.display(),
                                    e
                                );
                            }
                        }
                    }
                    Err(e) => {
                        // Warn about individual entry read errors but continue
                        eprintln!("Warning: Error reading directory entry in {}: {} (skipping entry)", dir, e);
                    }
                }
            }
        }
        Err(e) => {
            // Handle directory read errors gracefully
            eprintln!(
                "Warning: Cannot read directory '{}': {} (skipping directory)",
                dir, e
            );
            
            // Provide helpful guidance based on error type
            match e.kind() {
                std::io::ErrorKind::PermissionDenied => {
                    eprintln!("   Check directory permissions");
                }
                std::io::ErrorKind::NotFound => {
                    eprintln!(" Directory does not exist, will be created if needed");
                }
                std::io::ErrorKind::NotADirectory => {
                    eprintln!("   Path exists but is not a directory");
                }
                _ => {}
            }
        }
    }
}

/// Ask user how many files to display in results (1-100)
pub fn get_display_count() -> usize {
    loop {
        // Check for cancellation before prompting
        if CANCEL_REQUESTED.load(Ordering::SeqCst) {
            return 0;
        }
        
        print!("\nHow many file results to display? (1-200): ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_ok() {
            if let Ok(count) = input.trim().parse::<usize>() {
                if count >= 1 && count <= 1000 {
                    return count;
                }
            }
        }
        
        // Check for cancellation after input
        if CANCEL_REQUESTED.load(Ordering::SeqCst) {
            return 0;
        }
        
        println!("Invalid input. Please enter a number between 1 and 100.");
    }
}

/// Helper function to write output to both console and file
fn output_to_console_and_file(text: &str) -> std::io::Result<()> {
    logger::log(text)?;
    Ok(())
}

/// Helper function for formatted output to console and file
fn format_output_to_console_and_file(text: String) -> std::io::Result<()> {
    logger::logf(text)?;
    Ok(())
}

/// Format progress bar with percentage
fn format_progress_bar(completed: usize, total: usize, width: usize) -> String {
    if total == 0 {
        return "[".to_string() + &"=".repeat(width) + "] 0%";
    }
    
    let percentage = (completed as f64 / total as f64) * 100.0;
    let filled = (percentage as usize / 100) * width;
    let empty = width.saturating_sub(filled);
    
    format!(
        "[{}{}] {:.1}% ({}/{})",
        "=".repeat(filled),
        " ".repeat(empty),
        percentage,
        completed,
        total
    )
}

/// Display periodic progress updates with real-time metrics (single line update)
fn progress_reporter(tracker: SharedProgressTracker, update_interval: Duration) {
    let mut last_count = 0;
    
    loop {
        thread::sleep(update_interval);
        
        if let Some(summary) = tracker.get_summary() {
            // Only print if progress has changed
            if summary.files_completed != last_count {
                let bar = format_progress_bar(summary.files_completed, summary.total_files, 40);
                // Use ANSI escape codes to move cursor to beginning of line and clear it
                print!("\r\x1B[K{} | Elapsed: {:.1}s", bar, summary.elapsed_time.as_secs_f64());
                io::stdout().flush().unwrap();
                last_count = summary.files_completed;
            }
            
            // Exit when all files have been processed OR cancellation is requested
            if summary.files_completed >= summary.total_files || tracker.is_cancelled() {
                println!(); // Add newline after progress bar completes
                break;
            }
        }
    }
}

/// Display final results and statistics
pub fn display_results(tracker: SharedProgressTracker, display_count: usize, total_discovered: usize) {
    // Write to both console and file
    let _ = output_to_console_and_file(&format!("\n{:=<80}", ""));
    
    if tracker.is_cancelled() {
        let _ = output_to_console_and_file("Processing Cancelled by User");
    } else {
        let _ = output_to_console_and_file("Processing Complete!");
    }
    
    let _ = output_to_console_and_file(&format!("{:=<80}", ""));
    
    if let Some(summary) = tracker.get_summary() {
        let _ = output_to_console_and_file("");
        let _ = output_to_console_and_file("Final Statistics:");
        let _ = format_output_to_console_and_file(format!("  Total Files Discovered: {}", total_discovered));
        let _ = format_output_to_console_and_file(format!("  Total Files Analyzed: {}", summary.files_completed));
        let _ = format_output_to_console_and_file(format!("  Successfully Analyzed: {}", summary.files_completed - summary.total_errors));
        let _ = format_output_to_console_and_file(format!("  Total Errors: {}", summary.total_errors));
        let _ = format_output_to_console_and_file(format!("  Total Time: {:.2}s", summary.elapsed_time.as_secs_f64()));
        let _ = output_to_console_and_file("");
    }
    
    // Display sample results from user-specified number of files
    if let Some(analyses) = tracker.get_completed_analyses() {
        let actual_count = analyses.len();
        let show_count = std::cmp::min(display_count, actual_count);
        
        if show_count == 0 {
            let _ = output_to_console_and_file("No files to display.");
            println!("\nFull results saved to: {}\n\n", logger::output_path());
            return;
        }
        
        let _ = format_output_to_console_and_file(format!("Results ({} of {} files):", show_count, actual_count));
        let _ = output_to_console_and_file(&format!("{:-<80}", ""));
        
        for analysis in analyses.iter().take(show_count) {
            let _ = format_output_to_console_and_file(format!("\nFile: {}", analysis.filename));
            let _ = format_output_to_console_and_file(format!("  Status: {}", if analysis.is_successful() { " Success" } else { " Errors!!!" }));
            let _ = format_output_to_console_and_file(format!("  Processing Time: {:.3}ms", analysis.processing_time.as_secs_f64() * 1000.0));
            let _ = format_output_to_console_and_file(format!("  File Size: {} bytes", analysis.stats.size_bytes));
            let _ = format_output_to_console_and_file(format!("  Lines: {}", analysis.stats.line_count));
            let _ = format_output_to_console_and_file(format!("  Words: {}", analysis.stats.word_count));
            
            if analysis.stats.line_count > 0 {
                let _ = format_output_to_console_and_file(format!("  Avg Words per Line: {:.2}", 
                    analysis.stats.word_count as f64 / analysis.stats.line_count as f64));
            }
            
            // Show top 5 most frequent characters
            if !analysis.stats.char_frequencies.is_empty() {
                let mut chars: Vec<_> = analysis.stats.char_frequencies.iter().collect();
                chars.sort_by(|a, b| b.1.cmp(a.1));
                
                let _ = output_to_console_and_file("  Top Characters:");
                for (ch, count) in chars.iter().take(5) {
                    let ch_display = match *ch {
                        ' ' => "[space]".to_string(),
                        '\n' => "[newline]".to_string(),
                        '\t' => "[tab]".to_string(),
                        c => c.to_string(),
                    };
                    let _ = format_output_to_console_and_file(format!("    {} : {}", ch_display, count));
                }
            }
            
            // Display errors or success message
            if !analysis.errors.is_empty() {
                let _ = output_to_console_and_file("  Errors:");
                for error in &analysis.errors {
                    let _ = format_output_to_console_and_file(format!("    - {}", error));
                }
            } else {
                let _ = output_to_console_and_file("  No errors while processing file");
            }
        }
    }
    
    // Display error summary
    if let Some(errors) = tracker.get_errors() {
        if !errors.is_empty() {
            let _ = output_to_console_and_file(&format!("\n{:-<80}", ""));
            let _ = output_to_console_and_file("Error Summary:");
            for error in errors.iter().take(10) {
                let _ = format_output_to_console_and_file(format!("  - {}", error));
            }
            if errors.len() > 10 {
                let _ = format_output_to_console_and_file(format!("  ... and {} more errors", errors.len() - 10));
            }
        }
    }
    
    let _ = output_to_console_and_file(&format!("{:=<80}\n", ""));
    
    println!("\nFull results saved to: {}\n\n", logger::output_path());
}

pub fn read_int_from_user(prompt: &str) -> i32 {
    loop {
        // Check for cancellation before prompting
        if CANCEL_REQUESTED.load(Ordering::SeqCst) {
            return -1;
        }
        
        print!("{prompt}");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        if let Err(err) = io::stdin().read_line(&mut input) {
            eprintln!("Error reading input: {err}");
            continue;
        }

        let trimmed = input.trim();

        if trimmed.is_empty() {
            eprintln!("Please enter a number (no letters, no decimals).");
            continue;
        }

        // Attempt to parse as i32
        match trimmed.parse::<i32>() {
            Ok(num) => return num,
            Err(_) => {
                eprintln!("Invalid input. Enter an integer like 0, 5, -3 (no decimals or extra chars).");
            }
        }
    }
}


/// Process files with a given thread pool configuration
/// Returns (progress_tracker, total_discovered_files)
pub fn process_files(
    config: &ProcessorConfig,
    directories: &[String],
    extensions: &[String],
    limit: usize,
) -> (SharedProgressTracker, usize) {
    // Discover files
    println!("\nDiscovering files...");
    let files = discover_files(directories, extensions);
    let total_discovered = files.len();
    println!("Found {} files", total_discovered);
    
    if files.is_empty() {
        println!("\nNo files found.");
        return (SharedProgressTracker::new(), 0);
    }
    
    // Limit the number of files to process
    let files_to_process: Vec<String> = files.into_iter().take(limit).collect();
    let actual_count = files_to_process.len();
    
    if limit < total_discovered {
        println!("Processing {} files (limited from {} discovered files)", actual_count, total_discovered);
    } else {
        println!("Processing {} files", actual_count);
    }
    
    // Create thread pool and progress tracker
    let pool = ThreadPool::new(config.num_workers);
    let progress = SharedProgressTracker::new();
    progress.set_total_files(actual_count);
    
    println!("\nStarting processing with {} worker threads...", config.num_workers);
    println!("Press Ctrl+C to safely cancel processing\n", );
    
    // Start progress reporter thread
    let progress_clone = progress.clone();
    let reporter_handle = thread::spawn(move || {
        progress_reporter(progress_clone, Duration::from_secs(1));
    });
    
    // Submit tasks to thread pool (only process the limited number of files)
    for file_path in files_to_process {
        let progress_inner = progress.clone();
        
        pool.execute(move || {
            // Check for global cancellation flag
            if CANCEL_REQUESTED.load(Ordering::SeqCst) {
                return;
            }
            
            // Analyze file
            match FileAnalyzer::analyze_file(&file_path) {
                Ok(analysis) => {
                    // Check for cancellation before recording
                    if !CANCEL_REQUESTED.load(Ordering::SeqCst) {
                        // Record completion with lock
                        if let Ok(mut tracker) = progress_inner.clone_inner().lock() {
                            tracker.record_completion(analysis);
                        }
                    }
                }
                Err(e) => {
                    // Check for cancellation before recording error
                    if !CANCEL_REQUESTED.load(Ordering::SeqCst) {
                        // Create analysis with error
                        let mut analysis = FileAnalysis::new(file_path);
                        analysis.add_error(e);
                        if let Ok(mut tracker) = progress_inner.clone_inner().lock() {
                            tracker.record_completion(analysis);
                        }
                    }
                }
            }
        });
    }
    
    // Shutdown thread pool (wait for all tasks to complete)
    pool.shutdown();
    
    // Wait for progress reporter to finish (stops when all files processed)
    let _ = reporter_handle.join();
    
    // Check if processing was cancelled and update the progress tracker
    if CANCEL_REQUESTED.load(Ordering::SeqCst) {
        progress.cancel();
    }
    
    (progress, total_discovered)
}
