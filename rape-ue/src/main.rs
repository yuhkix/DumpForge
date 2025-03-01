use std::fs::File;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use goblin::pe::import::SyntheticImportLookupTableEntry;
use pe_utils::{parse_headers, get_optional_headers, get_sections, get_imports};
use aes_key_finder::{dump_aes_key, dump_aes_key_restricted};
use restorer::restore_from_dump;
use colored::*;
use serde_json::json;
use winconsole::console::set_title;
use std::env;
use winconsole::console::clear;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    set_title("Unreal Engine AES Fetcher").unwrap();

    loop {
        clear().expect("failed to clear console.");
        println!("{}", "Select an option:".bright_blue());
        println!("1. Fetch AES Key (Unreal Engine)");
        println!("2. Restore Section Headers");
        println!("3. Get Imports");
        println!("4. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;
        let choice = choice.trim();

        match choice {
            "1" => fetch_aes_key()?,
            "2" => restore_section_headers()?,
            "3" => get_executable_imports()?,
            "4" => break,
            _ => println!("{}", "Invalid choice. Please try again.".red().bold()),
        }

        println!("{}", "Do you want to return to the menu? (y/n):".bright_blue());
        let mut return_to_menu = String::new();
        io::stdin().read_line(&mut return_to_menu)?;
        let return_to_menu = return_to_menu.trim().to_lowercase();

        if return_to_menu != "y" {
            break;
        }
    }

    println!("{}", "Exiting program...".bright_blue());
    Ok(())
}

fn fetch_aes_key() -> Result<(), Box<dyn std::error::Error>> {
    clear().expect("failed to clear console.");
    println!("{}", "Enter the path to the executable:".bright_blue());

    let mut executable_path = String::new();
    io::stdin().read_line(&mut executable_path)?;
    let executable_path = executable_path.trim();

    if !Path::new(executable_path).exists() {
        eprintln!("{}: {}", "Error".red().bold(), format!("The specified path does not exist: {}", executable_path).red());
        println!("{}", "Press Enter to continue...".bright_blue());
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

    Ok(())
}

fn restore_section_headers() -> Result<(), Box<dyn std::error::Error>> {
    clear().expect("failed to clear console.");
    println!("{}", "Enter the path to the executable:".bright_blue());

    let mut executable_path = String::new();
    io::stdin().read_line(&mut executable_path)?;
    let executable_path = executable_path.trim();

    if !Path::new(executable_path).exists() {
        eprintln!("{}: {}", "Error".red().bold(), format!("The specified path does not exist: {}", executable_path).red());
        println!("{}", "Press Enter to continue...".bright_blue());
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        return Ok(());
    }

    let mut file = File::open(executable_path)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    let path_buf = PathBuf::from(executable_path);
    let executable_name = path_buf
        .file_stem()
        .and_then(|name| name.to_str())
        .unwrap_or("executable");

    let current_exe_dir = env::current_exe()?
        .parent()
        .ok_or("Failed to get current executable directory")?
        .to_path_buf();

    let new_name = format!("{}_restored.exe", executable_name);
    let restored_filename = current_exe_dir.join(new_name).to_string_lossy().into_owned();

    let restored_executable = restore_from_dump("restored_executable", &data, Some(restored_filename.clone()))?;

    let mut file = File::create(&restored_filename)?;
    file.write_all(&restored_executable)?;
    println!("Restored executable saved to: {}", restored_filename.bright_red());

    println!("{}", "Executable restored successfully.".bright_green().bold());

    Ok(())
}

fn get_executable_imports() -> Result<(), Box<dyn std::error::Error>> {
    clear().expect("failed to clear console.");
    println!("{}", "Enter the path to the executable:".bright_blue());

    let mut executable_path = String::new();
    io::stdin().read_line(&mut executable_path)?;
    let executable_path = executable_path.trim();

    if !Path::new(executable_path).exists() {
        eprintln!("{}: {}", "Error".red().bold(), format!("The specified path does not exist: {}", executable_path).red());
        println!("{}", "Press Enter to continue...".bright_blue());
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

    let imports = get_imports(&data, &optional_header, &sections)?;
    if let Some(import_data) = imports {

        // Prepare JSON output
        let mut json_output = json!({});

        for (_index, import) in import_data.import_data.iter().enumerate() {
            let library_name = import.name;
            let mut functions = Vec::new();
        
            if let Some(lookup_table) = &import.import_lookup_table {
                for (func_index, entry) in lookup_table.iter().enumerate() {
                    match entry {
                        SyntheticImportLookupTableEntry::OrdinalNumber(ordinal) => {
                            functions.push(json!({
                                "index": func_index + 1,
                                "type": "ordinal",
                                "ordinal": ordinal,
                            }));
                        }
                        SyntheticImportLookupTableEntry::HintNameTableRVA((_rva, hint_entry)) => {
                            functions.push(json!({
                                "index": func_index + 1,
                                "type": "name",
                                "name": hint_entry.name,
                                "hint": hint_entry.hint,
                            }));
                        }
                    }
                }
            }
        
            json_output[library_name] = json!(functions);
        }

        // Save JSON to file
        let path_buf = PathBuf::from(executable_path);
        let executable_name = path_buf
            .file_stem()
            .and_then(|name| name.to_str())
            .unwrap_or("imports");

        let imports_filename = format!("{}_imports.json", executable_name);
        let mut output_file = File::create(&imports_filename)?;
        writeln!(output_file, "{}", serde_json::to_string_pretty(&json_output)?)?;
        println!("Imports saved to: {}", imports_filename.bright_red());
    } else {
        println!(
            "{}",
            "No imports found in the executable.".red().bold()
        );
    }

    Ok(())
}