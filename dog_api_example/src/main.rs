use serde::Deserialize;
use std::error::Error;
use std::fmt;
use std::fs;
use std::io;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
struct DogImage {
    message: String,
    status: String,
}

// Custom error enum for the application
#[derive(Debug)]
enum AppError {
    /// Non-successful HTTP status code
    Http(u16),
    /// Network/transport-level error
    Transport(String),
    /// JSON parsing/serialization error
    Json(String),
    /// Filesystem I/O error
    Io(io::Error),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Http(code) => write!(f, "HTTP error: {}", code),
            AppError::Transport(msg) => write!(f, "Network/transport error: {}", msg),
            AppError::Json(msg) => write!(f, "JSON error: {}", msg),
            AppError::Io(e) => write!(f, "I/O error: {}", e),
        }
    }
}

impl Error for AppError {}

fn fetch_random_dog_image() -> Result<DogImage, AppError> {
    let url = "https://dog.ceo/api/breeds/image/random";
    
    match ureq::get(url).call() {
        Ok(response) => {
            if (200..=299).contains(&response.status()) {
                response
                    .into_json::<DogImage>()
                    .map_err(|e| AppError::Json(format!("Failed to parse JSON: {}", e)))
            } else {
                Err(AppError::Http(response.status()))
            }
        }
        Err(ureq::Error::Status(code, _resp)) => Err(AppError::Http(code)),
        Err(e) => Err(AppError::Transport(e.to_string())),
    }
}

/// Download an image from a URL and save it to images/dog_{index}.<ext>
fn save_image(url: &str, index: usize) -> Result<PathBuf, AppError> {
    // Ensure images directory exists
    fs::create_dir_all("images").map_err(AppError::Io)?;

    // Fetch the image
    let resp = match ureq::get(url).call() {
        Ok(r) => r,
        Err(ureq::Error::Status(code, _)) => return Err(AppError::Http(code)),
        Err(e) => return Err(AppError::Transport(e.to_string())),
    };

    if !(200..=299).contains(&resp.status()) {
        return Err(AppError::Http(resp.status()));
    }

    // Guess file extension from Content-Type
    let content_type = resp.header("Content-Type").unwrap_or("image/jpeg");
    let ext = if content_type.contains("png") {
        "png"
    } else if content_type.contains("gif") {
        "gif"
    } else if content_type.contains("webp") {
        "webp"
    } else {
        "jpg"
    };

    let path = PathBuf::from(format!("images/dog_{}.{}", index, ext));
    let mut reader = resp.into_reader();
    let mut file = fs::File::create(&path).map_err(AppError::Io)?;
    io::copy(&mut reader, &mut file).map_err(AppError::Io)?;
    Ok(path)
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Dog Image Fetcher");
    println!("=================\n");

    for i in 1..=5 {
        println!("Fetching random dog image #{}", i);
        match fetch_random_dog_image() {
            Ok(dog_image) => {
                println!("Success! JSON status: {}", dog_image.status);
                match save_image(&dog_image.message, i) {
                    Ok(path) => println!("Downloaded to: {}", path.display()),
                    Err(e) => println!("Failed to save image: {}", e),
                }
            }
            Err(e) => println!("Failed to fetch metadata: {}", e),
        }
        println!();
    }

    Ok(())
}