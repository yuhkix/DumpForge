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
use winconsole::console::clear;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    set_title("Wuthering Waves AES Fetcher").unwrap();

    clear().expect("failed to clear console.");
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
        println!("{}: {}", "AES key for main found".bold(), hex_key.bright_green().bold());
    } else {
        let aes_keys_relaxed = dump_aes_key(image_base, &sections, &data)?;
        if !aes_keys_relaxed.is_empty() {
            let hex_key: String = format!("0x{}", aes_keys_relaxed.iter().next().unwrap()
                .iter().map(|byte| format!("{:02X}", byte)).collect::<String>());
            json_output["aes_keys"]["main"] = json!(hex_key);
            println!("{}: {}", "AES key for main found".bold(), hex_key.bright_yellow().bold());
        } else {
            println!("{}", "No AES keys found for main in the executable.".red().bold());
        }
    }
    
    let path_buf = PathBuf::from(executable_path);

    let executable_name = path_buf
        .file_stem()
        .and_then(|name| name.to_str())
        .unwrap_or("AES");

    let aes_filename = format!("{}_AES.json", executable_name);

    let mut output_file = File::create(&aes_filename)?;
    writeln!(output_file, "{}", serde_json::to_string_pretty(&json_output)?)?;
    println!("AES key saved to: {}", aes_filename.bright_red());

    println!("{}", "Do you want to restore the executable? (y/n):".bright_blue());
    let mut restore_choice = String::new();
    io::stdin().read_line(&mut restore_choice)?;
    let restore_choice = restore_choice.trim().to_lowercase();

    if restore_choice == "y" {
        clear().expect("failed to clear console.");
        
        let current_exe_dir = env::current_exe()?
            .parent()
            .ok_or("Failed to get current executable directory")?
            .to_path_buf();

        let mut dump_file = File::open(executable_path)?;
        let mut dump_data = Vec::new();
        dump_file.read_to_end(&mut dump_data)?;

        let executable_name = path_buf
            .file_stem()
            .and_then(|name| name.to_str())
            .unwrap_or("executable");

        let new_name = format!("{}_restored.exe", executable_name);
        let restored_filename = current_exe_dir.join(new_name).to_string_lossy().into_owned();

        let restored_executable = restore_from_dump("restored_executable", &dump_data, Some(restored_filename.clone()))?;

        let mut file = File::create(&restored_filename)?;
        file.write_all(&restored_executable)?;
        println!("Restored executable saved to: {}", restored_filename.bright_red());

        println!("{}", "Executable restored successfully.".bright_green().bold());
    } else {
        clear().expect("failed to clear console.");
        println!("{}", "Executable restoration skipped.".bright_yellow().bold());
    }

    println!("{}", "Press Enter to exit...".bright_blue());
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    Ok(())
}