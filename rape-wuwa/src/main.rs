use std::fs::File;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use pe_utils::{parse_headers, get_optional_headers, get_sections};
use aes_key_finder::{dump_aes_key, dump_aes_key_restricted};
use restorer::restore_from_dump;
use colored::*;
use serde_json::json;
use winconsole::console::set_title;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    set_title("Wuthering Waves AES Fetcher").unwrap();

    println!("{}", "Enter the path to the executable:".bright_blue());

    let mut executable_path = String::new();
    io::stdin().read_line(&mut executable_path)?;
    let executable_path = executable_path.trim();

    if !Path::new(executable_path).exists() {
        eprintln!("{}: {}", "Error".red().bold(), format!("The specified path does not exist: {}", executable_path).red());
        println!("{}", "Press Enter to exit...".bright_blue());
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        return Ok(());
    }

    let mut file = File::open(executable_path)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    let header = parse_headers(&data)?;
    let optional_header = get_optional_headers(&header)?;
    let sections = get_sections(&header, &data)?;

    let image_base = optional_header.windows_fields.image_base as usize;

    let mut json_output = json!({
        "aes_keys": {
            "main": null,
            //"pakchunk-26": null,
            //"pakchunk-27": null,
            //"pakchunk-28": null
        }
    });

    let aes_keys_restricted = dump_aes_key_restricted(image_base, &sections, &data)?;
    if !aes_keys_restricted.is_empty() {
        let hex_key: String = format!("0x{}", aes_keys_restricted.iter().next().unwrap()
            .iter().map(|byte| format!("{:02X}", byte)).collect::<String>());
        json_output["aes_keys"]["main"] = json!(hex_key);
        println!("{}: {}", "AES key for main found and written to AES.json".bold(), hex_key.bright_green().bold());
    } else {
        let aes_keys_relaxed = dump_aes_key(image_base, &sections, &data)?;
        if !aes_keys_relaxed.is_empty() {
            let hex_key: String = format!("0x{}", aes_keys_relaxed.iter().next().unwrap()
                .iter().map(|byte| format!("{:02X}", byte)).collect::<String>());
            json_output["aes_keys"]["main"] = json!(hex_key);
            println!("{}: {}", "AES key for main found and written to AES.json".bold(), hex_key.bright_yellow().bold());
        } else {
            println!("{}", "No AES keys found for main in the executable.".red().bold());
        }
    }

<<<<<<< Updated upstream
    let mut output_file = File::create("AES.json")?;
    writeln!(output_file, "{}", serde_json::to_string_pretty(&json_output)?)?;
    
    println!("{}", "Press Enter to exit...".bright_blue());
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
=======
    let path_buf = PathBuf::from(executable_path);

    let executable_name = path_buf
        .file_stem()
        .and_then(|name| name.to_str())
        .unwrap_or("AES");

    let aes_filename = format!("{}_AES.json", executable_name);

    let mut output_file = File::create(&aes_filename)?;
    writeln!(output_file, "{}", serde_json::to_string_pretty(&json_output)?)?;
    println!("AES key data saved to: {}", aes_filename);

    // Ask the user if they want to restore the executable
    println!("{}", "Do you want to restore the executable? (y/n):".bright_blue());
    let mut restore_choice = String::new();
    io::stdin().read_line(&mut restore_choice)?;
    let restore_choice = restore_choice.trim().to_lowercase();

    if restore_choice == "y" {
        let mut dump_file = File::open(executable_path)?;
        let mut dump_data = Vec::new();
        dump_file.read_to_end(&mut dump_data)?;

        println!("{}", "Enter the path to save the restored executable (optional, leave empty for default):".bright_blue());
        let mut restored_path = String::new();
        io::stdin().read_line(&mut restored_path)?;
        let restored_path = restored_path.trim();

        let current_dir = env::current_dir()?;

        let restored_filename = if restored_path.is_empty() {
            let path_buf = PathBuf::from(executable_path);
            
            let executable_name = path_buf
                .file_stem()
                .and_then(|name| name.to_str())
                .unwrap_or("executable");

            let new_name = format!("{}_restored.exe", executable_name);
            Some(current_dir.join(new_name).to_string_lossy().into_owned())
        } else {
            Some(current_dir.join(restored_path).to_string_lossy().into_owned())
        };

        let restored_executable = restore_from_dump("restored_executable", &dump_data, restored_filename.clone())?;

        if let Some(filename) = restored_filename {
            let mut file = File::create(&filename)?;
            file.write_all(&restored_executable)?;
            println!("Restored executable saved to: {}", filename);
        }

        println!("{}", "Executable restored successfully.".bright_green().bold());
    } else {
        println!("{}", "Executable restoration skipped.".bright_yellow().bold());
    }

    println!("{}", "Press Enter to exit...".bright_blue());
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

>>>>>>> Stashed changes
    Ok(())
}