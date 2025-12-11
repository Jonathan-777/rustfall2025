use final_project::main_helper::{
    start_ctrlc_listener, ProcessorConfig, get_directories_from_user,
    get_display_count, display_results, read_int_from_user, process_files
};

use final_project::download_books_to_meet_demand;

fn main() {
    // Initialize logger - clear previous output file
    if let Err(e) = final_project::logger::init() {
        eprintln!("Warning: Could not initialize logger: {}", e);
    }
    
    // Start listening for Ctrl+C at program start, if our user clicks ctrl+c program gracefully terminates
    start_ctrlc_listener();//Asynchronous
    
    println!("Parallel File Processor Started");
    println!("{:*<80}", "");
    
    let user_integer_input = read_int_from_user("How many worker threads do you want?  Please Enter and integer ONLY : ");
    
    // Check if cancelled during input (sentinel value -1)
    if user_integer_input < 1 {
        println!("\n Cancelled. Exiting...");
        return;
    }
    
    // Get directories from user
    println!("\n{:*<80}", "");
    println!(" Configure Directories");
    println!("{:*<80}", "");
    let mut config = ProcessorConfig::new(user_integer_input as usize);
    config.directories = get_directories_from_user();
    
    println!("\nCurrent Configuration:");
    println!("  Worker Threads: {}", config.num_workers);
    println!("  Directories: {:?}", config.directories);
    println!("  File Extensions: {:?}", config.extensions);
    
    // Ask user how many results to display/process
    let display_count = get_display_count();
    
    // Check if cancelled during input (sentinel value 0)
    if display_count == 0 {
        println!("\n Cancelled. Exiting...");
        return;
    }
    
    // Process only the requested number of files
    let (mut progress, mut total_discovered) = process_files(&config, &config.directories, &config.extensions, display_count);
    
    // If processing was cancelled, skip the display and exit gracefully
    if progress.is_cancelled() {
        println!("\n Cancellation completed. Exiting...");
        return;
    }
    
    // Check if we need to download more files
    if let Some(analyses) = progress.get_completed_analyses() {
        let actual_count = analyses.len();
        
        if display_count > actual_count {
            println!("\n{:=<80}", "");
            println!("Not enough processed files!");
            println!("  Processed: {}, Requested: {}", actual_count, display_count);
            println!("{:=<80}", "");
            
            // Download more files
            match download_books_to_meet_demand("./books", actual_count, display_count) {
                Ok(_) => {
                    println!("\nRe-discovering files after download...");
                    
                    // Re-process with the requested limit
                    let (new_progress, new_total) = process_files(&config, &config.directories, &config.extensions, display_count);
                    progress = new_progress;
                    total_discovered = new_total;
                    
                    // Check if second run was cancelled
                    if progress.is_cancelled() {
                        println!("\n Cancellation completed. Exiting...");
                        return;
                    }
                }
                Err(e) => {
                    eprintln!("Error downloading files: {}", e);
                    eprintln!("Proceeding with available files...");
                }
            }
        }
    }
    
    // Display final results
    display_results(progress, display_count, total_discovered);
}
