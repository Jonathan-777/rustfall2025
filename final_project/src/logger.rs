use std::io::{self, Write, BufWriter};
use std::fs;
use std::sync::Mutex;
use std::sync::OnceLock;

const OUTPUT_FILE_PATH: &str = "./src/analysis_results.txt";

// Global file writer - initialized on first use
static FILE_WRITER: OnceLock<Mutex<BufWriter<fs::File>>> = OnceLock::new();

/// Initialize the logger by clearing the output file
pub fn init() -> io::Result<()> {
    // Clear the file at program start
    fs::File::create(OUTPUT_FILE_PATH)?;
    
    // Initialize the global file writer
    let file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(OUTPUT_FILE_PATH)?;
    let writer = BufWriter::new(file);
    let _ = FILE_WRITER.set(Mutex::new(writer));
    
    Ok(())
}

/// Write text to both console and file
pub fn log(text: &str) -> io::Result<()> {
    println!("{}", text);
    
    if let Some(writer_mutex) = FILE_WRITER.get() {
        if let Ok(mut writer) = writer_mutex.lock() {
            writeln!(writer, "{}", text)?;
            writer.flush()?;
        }
    }
    
    Ok(())
}

/// Write formatted text to both console and file
pub fn logf(text: String) -> io::Result<()> {
    println!("{}", text);
    
    if let Some(writer_mutex) = FILE_WRITER.get() {
        if let Ok(mut writer) = writer_mutex.lock() {
            writeln!(writer, "{}", text)?;
            writer.flush()?;
        }
    }
    
    Ok(())
}

/// Get output file path for confirmation message
pub fn output_path() -> &'static str {
    OUTPUT_FILE_PATH
}

