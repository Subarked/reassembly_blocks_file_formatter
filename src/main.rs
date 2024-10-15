use regex::Regex;
use std::env;
use std::fs::{self, OpenOptions};
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    // Get the file path from the command line arguments
    let args: Vec<String> = env::args().collect();

    // Ensure a file was provided
    if args.len() != 2 {
        eprintln!("Usage: drag and drop a file onto the executable.");
        return Ok(());
    }

    let file_path = &args[1];

    // Open the input file for reading
    let input_file = OpenOptions::new().read(true).open(file_path)?;

    // Create a buffered reader
    let reader = BufReader::new(input_file);

    let mut lines = reader.lines();

    // Read the first line to get the starting ID
    let first_line = lines.next().unwrap()?; // Extract the first line
    let start_id: u32 = extract_start_id(&first_line).expect("Invalid format for starting ID");

    // Collect the modified lines into a vector
    let mut modified_content = Vec::new();

    let mut current_id = start_id;
    for line in lines {
        let line = line?;
        // Replace $ID$ with the current ID
        let formatted_line = line.replace("$ID$", &current_id.to_string());

        // Store the formatted line
        modified_content.push(formatted_line.clone());
        if line != formatted_line {
            // Increment the ID
            current_id += 1;
        }
    }

    // Write the modified content back to the same file
    fs::write(file_path, modified_content.join("\n"))?;

    println!("File has been processed and updated: {}", file_path);

    Ok(())
}

// Helper function to extract the starting ID from the first line
fn extract_start_id(line: &str) -> Option<u32> {
    // Match the pattern $[Num]$
    let start_pattern = r"\$\[(\d+)\]\$";
    let re = Regex::new(start_pattern).ok()?;
    let captures = re.captures(line)?;
    captures.get(1)?.as_str().parse::<u32>().ok()
}
