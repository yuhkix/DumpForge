use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use pe_utils::{parse_headers, get_optional_headers, get_sections};
use aes_key_finder::{dump_aes_key, dump_aes_key_restricted};
use colored::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Ask the user for the path to the executable
    println!("{}", "Enter the path to the executable:".bright_blue());

    let mut executable_path = String::new();
    io::stdin().read_line(&mut executable_path)?;

    // Remove any trailing newline characters from the input
    let executable_path = executable_path.trim();

    // Check if the path is valid
    if !Path::new(executable_path).exists() {
        eprintln!("{}: {}", "Error".red().bold(), format!("The specified path does not exist: {}", executable_path).red());
        wait_for_enter();
        return Ok(());
    }

    // Read the executable file into a byte vector
    let mut file = File::open(executable_path)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    // Parse the PE headers
    let header = parse_headers(&data)?;
    let optional_header = get_optional_headers(&header)?;
    let sections = get_sections(&header, &data)?;

    // Get the image base from the optional header
    let image_base = optional_header.windows_fields.image_base as usize;

    // Try to find the AES key using the restricted filter (more precise)
    let aes_keys_restricted = dump_aes_key_restricted(image_base, &sections, &data)?;

    // Open a file to write the AES key
    let mut output_file = File::create("AES.txt")?;

    // If keys are found by the restricted filter, write the first one (most likely correct)
    if !aes_keys_restricted.is_empty() {
        let hex_key: String = aes_keys_restricted
            .iter()
            .next()
            .unwrap() // Safe to unwrap because we checked for emptiness
            .iter()
            .map(|byte| format!("{:02X}", byte))
            .collect();
        writeln!(output_file, "AES: 0x{}", hex_key)?;
        println!(
            "{}: {}",
            "Most likely AES key written to AES.txt".bold(),
            format!("0x{}", hex_key).bright_green().bold()
        );
    } else {
        // Fall back to the relaxed filter if no keys are found by the restricted filter
        let aes_keys_relaxed = dump_aes_key(image_base, &sections, &data)?;
        if !aes_keys_relaxed.is_empty() {
            let hex_key: String = aes_keys_relaxed
                .iter()
                .next()
                .unwrap() // Safe to unwrap because we checked for emptiness
                .iter()
                .map(|byte| format!("{:02X}", byte))
                .collect();
            writeln!(output_file, "0x{}", hex_key)?;
            println!(
                "{}: {}",
                "Most likely AES key written to AES.txt".bold(),
                format!("0x{}", hex_key).bright_yellow().bold()
            );
        } else {
            println!("{}", "No AES keys found in the executable.".red().bold());
        }
    }

    wait_for_enter();

    Ok(())
}

/// Helper function to wait for the user to press Enter
fn wait_for_enter() {
    println!("{}", "Press Enter to exit...".bright_blue());
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}