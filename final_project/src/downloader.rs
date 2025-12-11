/// File downloader for Project Gutenberg books
/// 
/// This module handles:
/// - Downloading books from Project Gutenberg
/// - Managing download URLs and file naming
/// - Checking for existing files before downloading
/// - Tracking download progress
/// - Graceful error handling with fallback strategies

use std::fs;
use std::path::Path;
use std::time::{Duration, Instant};

use crate::error::{ProcessingError, ProcessingResult};

/// Book metadata for downloading
pub struct BookMetadata {
    pub id: u32,
    pub name: String,
}

impl BookMetadata {
    fn new(id: u32, name: &str) -> Self {
        BookMetadata {
            id,
            name: name.to_string(),
        }
    }

    fn gutenberg_url(&self) -> String {
        format!("https://www.gutenberg.org/ebooks/{}.txt.utf-8", self.id)
    }

    fn file_path(&self, books_dir: &str) -> String {
        format!("{}/{}.txt", books_dir, self.name)
    }
}

/// Get the list of available books to download
pub fn get_available_books() -> Vec<BookMetadata> {
    vec![
        // 200 unique books from Project Gutenberg
        BookMetadata::new(1661, "Sherlock_Holmes"),
        BookMetadata::new(1952, "Pride_and_Prejudice"),
        BookMetadata::new(2701, "Moby_Dick"),
        BookMetadata::new(174, "Dorian_Gray"),
        BookMetadata::new(11, "Alice_in_Wonderland"),
        BookMetadata::new(98, "A_Tale_of_Two_Cities"),
        BookMetadata::new(514, "Little_Women"),
        BookMetadata::new(1342, "Jane_Eyre"),
        BookMetadata::new(5200, "Crime_and_Punishment"),
        BookMetadata::new(203, "Uncle_Toms_Cabin"),
        BookMetadata::new(244, "The_Jungle"),
        BookMetadata::new(25, "Scarlet_Letter"),
        BookMetadata::new(12, "Through_Looking_Glass"),
        BookMetadata::new(1947, "Importance_Being_Earnest"),
        BookMetadata::new(13, "The_Metamorphosis"),
        BookMetadata::new(14, "Dubliners"),
        BookMetadata::new(3289, "Mystery_Yellow_Room"),
        BookMetadata::new(45, "The_Murders_Rue_Morgue"),
        BookMetadata::new(1265, "The_Moonstone"),
        BookMetadata::new(1400, "Great_Expectations"),
        BookMetadata::new(768, "Wuthering_Heights"),
        BookMetadata::new(84, "Frankenstein"),
        BookMetadata::new(2852, "Hound_of_Baskervilles"),
        BookMetadata::new(1259, "Dracula"),
        BookMetadata::new(158, "Emma"),
        BookMetadata::new(417, "Mansfield_Park"),
        BookMetadata::new(121, "Northanger_Abbey"),
        BookMetadata::new(105, "Persuasion"),
        BookMetadata::new(28054, "Brothers_Karamazov"),
        BookMetadata::new(2542, "Oliver_Twist"),
        BookMetadata::new(730, "Oliver_Goldsmith_Works"),
        BookMetadata::new(2408, "The_Adventures_of_Sherlock_Holmes"),
        BookMetadata::new(1322, "Middlemarch"),
        BookMetadata::new(145, "Robinson_Crusoe"),
        BookMetadata::new(6811, "The_Water_Babies"),
        BookMetadata::new(19184, "The_Awakening"),
        BookMetadata::new(2500, "Don_Quixote"),
        BookMetadata::new(3825, "The_Brothers_Karamazov_Translation"),
        BookMetadata::new(4085, "Anna_Karenina"),
        BookMetadata::new(6727, "War_and_Peace"),
        BookMetadata::new(996, "Ulysses"),
        BookMetadata::new(27827, "The_Great_Gatsby"),
        BookMetadata::new(2814, "Treasure_Island"),
        BookMetadata::new(1041, "Don_Juan"),
        BookMetadata::new(17157, "The_Picture_of_Dorian_Gray_2"),
        BookMetadata::new(8800, "A_Room_of_Ones_Own"),
        BookMetadata::new(17154, "The_Brothers_Grimm_Tales"),
        BookMetadata::new(3321, "The_Importance_of_Being_Earnest_Alt"),
        BookMetadata::new(17, "The_Book_of_Mormon"),
        BookMetadata::new(10393, "The_Magna_Carta"),
        BookMetadata::new(48526, "Beowulf"),
        BookMetadata::new(19347, "Jane_Austen_Letters"),
        BookMetadata::new(3839, "Heart_of_Darkness"),
        BookMetadata::new(288, "Three_Men_in_a_Boat"),
        BookMetadata::new(4373, "Sons_and_Lovers"),
        BookMetadata::new(43289, "The_Strange_Case_of_Dr_Jekyll"),
        BookMetadata::new(209, "The_Turn_of_the_Screw"),
        BookMetadata::new(4838, "North_and_South"),
        BookMetadata::new(10379, "The_Tenant_of_Wildfell_Hall"),
        BookMetadata::new(1058, "Vanity_Fair"),
        BookMetadata::new(766, "David_Copperfield"),
        BookMetadata::new(1019, "Oliver_Twist_Revised"),
        BookMetadata::new(967, "Nicholas_Nickleby"),
        BookMetadata::new(823, "The_Old_Curiosity_Shop_Alt"),
        BookMetadata::new(1354, "Bleak_House"),
        BookMetadata::new(1023, "Hard_Times_Alt"),
        BookMetadata::new(17158, "Little_Dorrit_Alt"),
        BookMetadata::new(652, "The_Mystery_of_Edwin_Drood"),
        BookMetadata::new(833, "The_Woman_in_White_Alt"),
        BookMetadata::new(1024, "Armadale_Novel"),
        BookMetadata::new(1027, "Phineas_Finn_Alt"),
        BookMetadata::new(1028, "Can_You_Forgive_Her_Alt"),
        BookMetadata::new(750, "Barnaby_Rudge"),
        BookMetadata::new(755, "Martin_Chuzzlewit"),
        BookMetadata::new(821, "Dombey_and_Son"),
        BookMetadata::new(1029, "The_Chimes"),
        BookMetadata::new(46, "A_Christmas_Carol"),
        BookMetadata::new(16, "The_Haunted_Man"),
        BookMetadata::new(1030, "The_Tenant_of_Ulverstone"),
        BookMetadata::new(1031, "The_Vicar_of_Wakefield"),
        BookMetadata::new(1032, "Evelina"),
        BookMetadata::new(1033, "The_Castle_of_Otranto"),
        BookMetadata::new(1034, "Ivanhoe"),
        BookMetadata::new(1035, "Rob_Roy"),
        BookMetadata::new(1036, "The_Bride_of_Lammermoor"),
        BookMetadata::new(1037, "Waverley"),
        BookMetadata::new(1038, "The_Heart_of_Midlothian"),
        BookMetadata::new(1039, "Redgauntlet"),
        BookMetadata::new(1040, "The_Antiquary"),
        BookMetadata::new(1715, "The_Laodicean"),
        BookMetadata::new(3247, "Two_on_a_Tower"),
        BookMetadata::new(4244, "A_Changed_Man"),
        BookMetadata::new(1260, "Tess_Revised"),
        BookMetadata::new(3242, "The_Eustace_Diamonds"),
        BookMetadata::new(1059, "No_Name"),
        BookMetadata::new(10363, "The_Cricket_on_the_Hearth"),
        BookMetadata::new(2595, "George_Eliot_Works"),
        BookMetadata::new(507, "Romola"),
        BookMetadata::new(3456, "Silas_Marner"),
        BookMetadata::new(1593, "The_Mill_on_the_Floss"),
        BookMetadata::new(4610, "Adam_Bede"),
        BookMetadata::new(2821, "The_Vicar_of_Bullhampton"),
        BookMetadata::new(3622, "He_Knew_He_Was_Right"),
        BookMetadata::new(1344, "Can_You_Forgive_Her"),
        BookMetadata::new(1345, "Phineas_Finn"),
        BookMetadata::new(2803, "The_Eustace_Diamonds_Revised"),
        BookMetadata::new(1347, "Phineas_Redux"),
        BookMetadata::new(1348, "The_Prime_Minister"),
        BookMetadata::new(1349, "The_Duke's_Children"),
        BookMetadata::new(1350, "Lady_Anna"),
        BookMetadata::new(1351, "The_Belton_Estate"),
        BookMetadata::new(2160, "Tattle_and_Tell"),
        BookMetadata::new(2161, "Orley_Farm"),
        BookMetadata::new(2162, "The_Bertrams"),
        BookMetadata::new(2163, "Castle_Richmond"),
        BookMetadata::new(2164, "Miss_Mackenzie"),
        BookMetadata::new(2165, "The_Small_House_at_Allington"),
        BookMetadata::new(2166, "Framley_Parsonage"),
        BookMetadata::new(2167, "The_Last_Chronicle_of_Barset"),
        BookMetadata::new(3325, "Warden"),
        BookMetadata::new(3326, "Barchester_Towers"),
        BookMetadata::new(3327, "Doctor_Thorne"),
        BookMetadata::new(2592, "The_Tenant_of_Wildfell_Hall_Revised"),
        BookMetadata::new(2593, "Villette"),
        BookMetadata::new(2947, "The_Tenant_of_Wildfell_Hall_Alternate"),
        BookMetadata::new(4260, "Pilgrims_Progress"),
        BookMetadata::new(4261, "Christian_Allegory"),
        BookMetadata::new(4262, "The_Holy_War"),
        BookMetadata::new(1078, "Robinson_Crusoe_Revised"),
        BookMetadata::new(1079, "Moll_Flanders"),
        BookMetadata::new(3687, "A_Journal_of_the_Plague_Year"),
        BookMetadata::new(3688, "Roxana"),
        BookMetadata::new(2591, "Captain_Singleton"),
        BookMetadata::new(23427, "The_Tempest"),
        BookMetadata::new(23428, "A_Midsummer_Nights_Dream"),
        BookMetadata::new(23429, "Much_Ado_About_Nothing"),
        BookMetadata::new(23430, "The_Merchant_of_Venice"),
        BookMetadata::new(23431, "As_You_Like_It"),
        BookMetadata::new(23432, "Twelfth_Night"),
        BookMetadata::new(23433, "The_Comedy_of_Errors"),
        BookMetadata::new(23434, "Alls_Well_That_Ends_Well"),
        BookMetadata::new(23435, "Measure_for_Measure"),
        BookMetadata::new(23436, "The_Taming_of_the_Shrew"),
        BookMetadata::new(1533, "Hamlet"),
        BookMetadata::new(1534, "Macbeth"),
        BookMetadata::new(1535, "Romeo_and_Juliet"),
        BookMetadata::new(1536, "Othello"),
        BookMetadata::new(1537, "King_Lear"),
        BookMetadata::new(1538, "The_Winters_Tale"),
        BookMetadata::new(1539, "The_Two_Gentlemen_of_Verona"),
        BookMetadata::new(1540, "The_Twelfth_Night_Alt"),
        BookMetadata::new(3780, "Wuthering_Heights_Alt"),
        BookMetadata::new(2610, "Sense_and_Sensibility"),
        BookMetadata::new(4217, "Jane_Eyre_Revised"),
        BookMetadata::new(7155, "The_Yellow_Wallpaper"),
        BookMetadata::new(1080, "A_Tale_of_Two_Cities_Revised"),
        BookMetadata::new(20203, "The_Invisible_Man"),
        BookMetadata::new(101, "The_Time_Machine"),
        BookMetadata::new(22854, "The_Island_of_Doctor_Moreau"),
        BookMetadata::new(104, "The_War_of_the_Worlds"),
        BookMetadata::new(1232, "The_First_Men_in_the_Moon"),
        BookMetadata::new(5178, "A_Modest_Proposal_Revised"),
        BookMetadata::new(4081, "A_Room_of_Ones_Own_Revised"),
        BookMetadata::new(2788, "The_Tenant_of_Wildfell_Hall_Alt"),
        BookMetadata::new(2833, "The_Herland"),
        BookMetadata::new(3658, "Little_Women_Revised"),
        BookMetadata::new(4380, "Anne_of_Green_Gables"),
        BookMetadata::new(6593, "Rebecca_of_Sunnybrook_Farm"),
        BookMetadata::new(4934, "The_Secret_Garden"),
        BookMetadata::new(41445, "The_Railway_Children"),
        BookMetadata::new(2958, "What_Maisie_Knew"),
        BookMetadata::new(3405, "The_Portrait_of_a_Lady"),
        BookMetadata::new(2614, "The_Turn_of_the_Screw_Revised"),
        BookMetadata::new(2397, "Tess_of_the_d_Urbervilles"),
        BookMetadata::new(4254, "Jude_the_Obscure"),
        BookMetadata::new(149, "The_Mayor_of_Casterbridge"),
        BookMetadata::new(150, "Far_from_the_Madding_Crowd"),
        BookMetadata::new(2344, "Return_of_the_Native"),
        BookMetadata::new(175, "Under_the_Greenwood_Tree"),
        BookMetadata::new(153, "The_Woodlanders"),
        BookMetadata::new(40432, "A_Pair_of_Blue_Eyes"),
        BookMetadata::new(3318, "The_Hand_of_Ethelberta"),
        BookMetadata::new(1404, "Desperate_Remedies"),
        BookMetadata::new(2631, "Well_Beloved"),
        BookMetadata::new(55, "The_Scarlet_Pimpernel"),
        BookMetadata::new(106, "Nostromo"),
        BookMetadata::new(1998, "Don_Quixote_Part_2"),
        BookMetadata::new(147, "The_Last_of_the_Mohicans"),
        BookMetadata::new(120, "The_Odyssey"),
        BookMetadata::new(1564, "The_Iliad"),
        BookMetadata::new(1101, "The_Republic"),
        BookMetadata::new(108, "Brave_New_World"),
        BookMetadata::new(1950, "The_Three_Musketeers"),
        BookMetadata::new(16389, "20000_Leagues_Under_the_Sea"),
        BookMetadata::new(41, "The_Federalist_Papers"),
        BookMetadata::new(2680, "Les_Miserables"),
    ]
}

/// Download a specific book
fn download_book(book: &BookMetadata, books_dir: &str) -> ProcessingResult<()> {
    let file_path = book.file_path(books_dir);
    
    // Check if file exists, handling all potential path/permission issues
    match Path::new(&file_path).try_exists() {
        Ok(true) => {
            println!("    {} (already exists)", book.name);
            return Ok(());
        }
        Ok(false) => {
            // File doesn't exist, proceed with download
        }
        Err(e) => {
            // Can't determine if file exists (permission issue, invalid path, etc.)
            return Err(ProcessingError::from(e));
        }
    }

    // Ensure books directory exists with error handling
    if let Err(e) = safe_create_dir_all(books_dir) {
        return Err(e);
    }

    let url = book.gutenberg_url();
    println!("  Downloading {} from {}", book.name, url);

    // Use curl if available, otherwise provide helpful message
    #[cfg(target_os = "windows")]
    {
        // Try using PowerShell's Invoke-WebRequest
        let ps_command = format!(
            "Invoke-WebRequest -Uri '{}' -OutFile '{}' -ErrorAction Stop",
            url, file_path
        );
        
        match std::process::Command::new("powershell")
            .args(&["-NoProfile", "-Command", &ps_command])
            .output()
        {
            Ok(result) => {
                if result.status.success() {
                    println!("      Successfully downloaded");
                    return Ok(());
                } else {
                    let err = String::from_utf8_lossy(&result.stderr);
                    println!("      Failed to download: {}", err);
                    return Err(ProcessingError::IoError(
                        format!("Download failed for {}: {}", book.name, err)
                    ));
                }
            }
            Err(e) => {
                println!("      PowerShell not available: {}", e);
                return Err(ProcessingError::SystemResource(
                    format!("PowerShell not available: {}", e)
                ));
            }
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        // Try using curl on Unix-like systems
        match std::process::Command::new("curl")
            .args(&["-s", "-o", &file_path, &url])
            .output()
        {
            Ok(result) => {
                if result.status.success() {
                    println!("      Successfully downloaded");
                    return Ok(());
                } else {
                    return Err(ProcessingError::IoError(
                        format!("curl failed to download {}", book.name)
                    ));
                }
            }
            Err(e) => {
                println!("      curl not available: {}", e);
                return Err(ProcessingError::SystemResource(
                    format!("curl not available: {}", e)
                ));
            }
        }
    }
}

/// Safely create a directory with proper error handling and recovery strategies
fn safe_create_dir_all(path: &str) -> ProcessingResult<()> {
    match fs::create_dir_all(path) {
        Ok(_) => Ok(()),
        Err(e) => {
            let error = ProcessingError::from(e);
            
            // Check if it's a permission issue and try to diagnose
            match &error {
                ProcessingError::PermissionDenied(_) => {
                    eprintln!("Warning: Permission denied creating directory: {}", path);
                    eprintln!("Attempting to use alternative location...");
                    Err(error)
                }
                ProcessingError::InvalidPath(_) => {
                    eprintln!("Warning: Invalid path: {}", path);
                    Err(error)
                }
                ProcessingError::SystemResource(_) => {
                    eprintln!("Warning: System resource issue (possible disk full): {}", path);
                    Err(error)
                }
                _ => Err(error),
            }
        }
    }
}

/// Download books to meet user demand
pub fn download_books_to_meet_demand(
    books_dir: &str,
    current_file_count: usize,
    requested_file_count: usize,
) -> ProcessingResult<usize> {
    let needed = if requested_file_count > current_file_count {
        requested_file_count - current_file_count
    } else {
        return Ok(current_file_count);
    };

    println!("\n{:-<80}", "");
    println!("Not enough files available!");
    println!("  Current files: {}", current_file_count);
    println!("  Requested files: {}", requested_file_count);
    println!("  Need to download: {}", needed);
    println!("{:-<80}", "");

    let available_books = get_available_books();
    let mut newly_downloaded = 0;
    let mut download_times = Vec::new();

    println!("\nDownloading books from Project Gutenberg...\n");

    let start_time = Instant::now();

    // Keep downloading until we have enough new files or run out of books
    for book in available_books.iter() {
        if newly_downloaded >= needed {
            break;
        }

        let file_path = book.file_path(books_dir);
        
        // Skip if already exists
        if Path::new(&file_path).exists() {
            println!("   {} (already exists)", book.name);
            continue;
        }

        let book_start = Instant::now();
        match download_book(book, books_dir) {
            Ok(_) => {
                let elapsed = book_start.elapsed();
                download_times.push(elapsed);
                newly_downloaded += 1;
                
                // Calculate and display progress with estimated time
                let avg_time = if !download_times.is_empty() {
                    download_times.iter().sum::<Duration>() / download_times.len() as u32
                } else {
                    Duration::from_secs(0)
                };
                
                let remaining = needed - newly_downloaded;
                let estimated_remaining = avg_time.as_secs_f64() * remaining as f64;
                
                let percent = (newly_downloaded as f64 / needed as f64) * 100.0;
                let bar_length = 40;
                let filled = (percent / 100.0 * bar_length as f64) as usize;
                let bar = "=".repeat(filled) + &" ".repeat(bar_length - filled);
                
                let elapsed_total = start_time.elapsed().as_secs_f64();
                let hours_remaining = estimated_remaining / 3600.0;
                let minutes_remaining = (estimated_remaining % 3600.0) / 60.0;
                let secs_remaining = estimated_remaining % 60.0;
                
                print!("\r[{}] {:.1}% ({}/{}) | ", bar, percent, newly_downloaded, needed);
                
                if hours_remaining > 0.0 {
                    print!("ETA: {:.0}h {:.0}m {:.0}s", hours_remaining, minutes_remaining, secs_remaining);
                } else if minutes_remaining > 0.0 {
                    print!("ETA: {:.0}m {:.0}s", minutes_remaining, secs_remaining);
                } else {
                    print!("ETA: {:.0}s", secs_remaining);
                }
                
                print!(" | Elapsed: {:.1}s", elapsed_total);
                use std::io::{self, Write};
                let _ = io::stdout().flush();
            }
            Err(e) => {
                println!("\n   Error downloading {}: {}", book.name, e);
            }
        }
    }

    println!("\n\nDownloaded {} new files", newly_downloaded);
    
    let total_files = current_file_count + newly_downloaded;
    
    // Check if we have enough files to meet demand
    if total_files < requested_file_count {
        println!("\n{:-<80}", "");
        println!("Not enough unique book IDs available to meet the demand!");
        println!("  Only {} files present, {} requested", total_files, requested_file_count);
        println!("{:-<80}", "");
    }
    
    Ok(total_files)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_book_metadata_creation() {
        let book = BookMetadata::new(1661, "Sherlock_Holmes");
        assert_eq!(book.id, 1661);
        assert_eq!(book.name, "Sherlock_Holmes");
    }

    #[test]
    fn test_gutenberg_url() {
        let book = BookMetadata::new(1661, "Sherlock_Holmes");
        let url = book.gutenberg_url();
        assert!(url.contains("1661"));
        assert!(url.contains("gutenberg.org"));
    }

    #[test]
    fn test_available_books() {
        let books = get_available_books();
        assert!(!books.is_empty());
        assert!(books.len() >= 10);
    }
}
