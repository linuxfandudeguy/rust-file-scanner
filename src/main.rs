use colored::*;
use std::env;
use std::fs::{read_dir, File};
use std::io::Read;
use std::path::PathBuf;

fn main() {
    // Get directory path from command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <directory>", args[0]);
        std::process::exit(1);
    }

    let path = &args[1];
    let mut total_files = 0;
    let mut corrupted_files = Vec::new();

    // Start scanning
    match scan_directory(path) {
        Ok((file_count, corrupted)) => {
            total_files = file_count;
            corrupted_files = corrupted;
        }
        Err(e) => eprintln!("Error scanning directory: {}", e),
    }

    // Output results
    println!("Total files scanned: {}", total_files);
    if !corrupted_files.is_empty() {
        println!("Corrupted files:");
        for file in corrupted_files {
            println!("{}", file.display().to_string().red());  // Convert PathBuf to String and then color it
        }
    } else {
        println!("No corrupted files found.");
    }
}

// Function to scan a directory and identify corrupted files
fn scan_directory(dir: &str) -> Result<(usize, Vec<PathBuf>), std::io::Error> {
    let mut total_files = 0;
    let mut corrupted_files = Vec::new();

    for entry in read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            // Recursively scan directories
            let (file_count, mut corrupted) = scan_directory(path.to_str().unwrap())?;
            total_files += file_count;
            corrupted_files.append(&mut corrupted);
        } else {
            total_files += 1;
            if !check_file(&path) {
                corrupted_files.push(path);
            }
        }
    }

    Ok((total_files, corrupted_files))
}

// Function to check if a file is corrupted
fn check_file(path: &PathBuf) -> bool {
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(_) => return false,
    };

    let mut buffer = Vec::new();
    match file.read_to_end(&mut buffer) {
        Ok(_) => true,
        Err(_) => false,
    }
}
