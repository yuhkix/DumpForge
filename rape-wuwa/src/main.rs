use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use pe_utils::{parse_headers, get_optional_headers, get_sections};
use aes_key_finder::{dump_aes_key, dump_aes_key_restricted};
use colored::*;
use serde_json::json;
use winconsole::console::set_title;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    set_title("Wuthering Waves AES Fetcher").unwrap();

    println!("{}", "Enter the path to the executable:".bright_blue());
    
    let mut executable_path = String::new();
    io::stdin().read_line(&mut executable_path)?;
    let executable_path = executable_path.trim();

    if !Path::new(executable_path).exists() {
        eprintln!("{}: {}", "Error".red().bold(), format!("The specified path does not exist: {}", executable_path).red());
        wait_for_enter();
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
            "pakchunk-26": null,
            "pakchunk-27": null,
            "pakchunk-28": null
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

    let mut output_file = File::create("AES.json")?;
    writeln!(output_file, "{}", serde_json::to_string_pretty(&json_output)?)?;
    
    println!("{}", "Press Enter to exit...".bright_blue());
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    Ok(())
}