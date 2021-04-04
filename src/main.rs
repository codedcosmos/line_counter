use std::env;
use std::fs;
use std::collections::HashMap;
use std::fs::{DirEntry, FileType};
use std::io::Error;
use std::ffi::OsString;
use std::path::{PathBuf, Path};
use std::process::exit;

enum Following {
    None,
    Paths,
    Extensions,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("No arguments, you should run this program with both --paths and --extensions");
        return;
    }

    let mut paths = Vec::new();
    let mut extensions = Vec::new();

    let mut following = Following::None;
    for i in 1..args.len() {
        let arg = &args[i];

        if arg.eq("--paths") {
            following = Following::Paths;
            continue;
        } else if arg.eq("--extensions") {
            following = Following::Extensions;
            continue;
        }

        match following {
            Following::None => {
                println!("Don't know what argument '{}' is for, please start line_counters args with --paths or --extensions", arg);
                return;
            }
            Following::Paths => {
                paths.push(arg);
            }
            Following::Extensions => {
                extensions.push(arg);
            }
        }
    }

    if extensions.len() <= 0 {
        println!("No specified file extensions, specify with --extensions");
    }
    if paths.len() <= 0 {
        println!("No specified file paths, specify with --paths");
    }
    if extensions.len() <= 0 || paths.len() <= 0 {
        return;
    }

    // Keep track
    let mut num_lines_total: u128 = 0;
    let mut num_lines_by_extension: HashMap<String, u128> = HashMap::new();

    for extension in extensions {
        num_lines_by_extension.insert(extension.to_string(), 0);
    }

    // Recursively find files and count numbers
    for path in paths {
        find_files_in_path(&mut num_lines_total, &mut num_lines_by_extension, path);
    }

    // Display
    println!("Total number of lines:");
    for (key, value) in num_lines_by_extension {
        println!("{: <8}: {}", key, value);
    }
    println!("");
    println!("Total   : {}", num_lines_total);
}

fn find_files_in_path(mut num_lines_total: &mut u128, mut num_lines_by_extension: &mut HashMap<String, u128>, path: &String) {
    let paths = fs::read_dir(path).expect(format!("Failed to parse path {}", path).as_str());
    for path in paths {
        match path {
            Ok(dir_entry) => {
                match dir_entry.path().into_os_string().into_string() {
                    Ok(path) => {

                        match dir_entry.file_type() {
                            Ok(file_type) => {
                                if file_type.is_file() {
                                    // File logic
                                    add_count_from_file(&mut num_lines_total, &mut num_lines_by_extension, &path);
                                } else {
                                    // Recursively follow path
                                    find_files_in_path(&mut num_lines_total, &mut num_lines_by_extension, &path);
                                }
                            }
                            Err(e) => {
                                println!("Could not get file type, {}", e);
                            }
                        }

                    }
                    Err(e) => {
                        println!("Could not get entry path for file");
                    }
                }
            }
            Err(e) => {
                println!("Error occured with file, {}", e);
            }
        }
    }
}

fn add_count_from_file(mut num_lines_total: &mut u128, mut num_lines_by_extension: &mut HashMap<String, u128>, path: &String) {
    // Get extension
    let split = path.split(".").last();
    let mut extension = "";
    match split {
        Some(some) => {
            extension = some;
        }
        None => {
            println!("Could not get extension for {}", path);
            return;
        }
    }

    // Make sure extension is in check list
    match num_lines_by_extension.get_mut(extension) {
        Some(counter) => {

            // Count lines
            match fs::read_to_string(path) {
                Ok(content) => {
                    let mut num_lines: u128 = 0;
                    // Account for first line
                    num_lines += 1;

                    // Count others
                    for char in content.chars() {
                        if char == '\n' {
                            num_lines += 1;
                        }
                    }

                    // Add to total
                    *num_lines_total += num_lines;
                    *counter += num_lines;
                }
                Err(e) => {
                    println!("Error occured while counting lines for file, {}", e);
                }
            }

        }
        None => {
            // Ignore file since it shouldn't be counted
        }
    }
}