use std::fs::File;
use std::io::{Write, BufReader, BufRead};

// The Book struct requirements
struct Book {
    title: String,
    author: String,
    year: u16,
}

/// Saves a vector of Book structs to a file.
/// Each book is written on a new line in a comma-separated format
fn save_books(books: &Vec<Book>, filename: &str) {
    // 1. Create (or overwrite) file
    let mut file = File::create(filename)
        .expect("Failed to create file for saving books.");

    // 2. Iterate over the books and write them by line
    for book in books {
        let line = format!("{},{},{}\n", book.title, book.author, book.year);
        
        // write_all ensures the entire buffer is written
        file.write_all(line.as_bytes())
            .expect("Failed to write data to file.");
    }
}

/// Loads a vector of Book structs from a file.
// assumes each line is correctly formatted as "title,author,year".
fn load_books(filename: &str) -> Vec<Book> {
    //  Attempt to open the file. If it fails (e.g., file not found), return an empty Vec.
    let file = match File::open(filename) {
        Ok(f) => f,
        Err(_) => {
            println!("Book catalog file '{}' not found. Starting with an empty catalog.", filename);
            return Vec::new();
        }
    };

    let reader = BufReader::new(file);
    let mut books = Vec::new();

    //  Iterate through each line of the file
    for line_result in reader.lines() {
        let line = line_result.expect("Error reading line from file");
        
        // Trim whitespace and split the line by the comma delimiter
        let parts: Vec<&str> = line.trim().split(',').collect();

        // eensure we have exactly 3 parts
        if parts.len() == 3 {
            // Parse the year string into a u16
            let year = match parts[2].parse::<u16>() {
                Ok(y) => y,
                Err(_) => {
                    println!("Warning: Skipping malformed line (Year Parse Error): {}", line);
                    continue; // Skip this line and move to the next
                }
            };

            let book = Book {
                // Convert &str slices to owned Strings for the struct fields
                title: parts[0].to_string(),
                author: parts[1].to_string(),
                year,
            };
            books.push(book);
        } else {
            println!("Warning: Skipping malformed line (Incorrect format): {}", line);
        }
    }

    books
}

fn main() {
    //  Initial Data Creation 
    let books_to_save = vec![
        Book { title: "1984".to_string(), author: "George Orwell".to_string(), year: 1949 },
        Book { title: "To Kill a Mockingbird".to_string(), author: "Harper Lee".to_string(), year: 1960 },
        Book { title: "The 48 Laws of Power".to_string(), author: "Robert Greene".to_string(), year: 1998 },
    ];
    
    let filename = "books.txt";

    //  Save Books 
    println!("Saving initial book collection ({} books) to {}...", books_to_save.len(), filename);
    save_books(&books_to_save, filename);
    println!("Books successfully saved.");

    //  Load Books 
    println!("\n books from {}.", filename);
    let loaded_books = load_books(filename);
    
    //  Print Loaded Books 
    println!(" loaded {} books:", loaded_books.len());
    for book in loaded_books {
        println!("- {} by {}, published in {}", book.title, book.author, book.year);
    }
}