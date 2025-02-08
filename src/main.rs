use regex::Regex;
use std::env;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;

fn list_files_with_extensions(dir: &Path, extensions: &[&str]) -> io::Result<Vec<String>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            files.extend(list_files_with_extensions(&path, extensions)?);
        } else if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
            if extensions.contains(&ext) {
                files.push(path.to_string_lossy().to_string());
            }
        }
    }
    Ok(files)
}

fn merge_yaml_files(files: Vec<String>) -> Result<String, Box<dyn std::error::Error>> {
    let mut merged_yaml = Vec::new();
    for file in &files {
        let mut content = String::new();
        File::open(file)?.read_to_string(&mut content)?;
        merged_yaml.push((file, content));
    }
    let mut out_str = String::new();
    for (i, (file, docs)) in merged_yaml.iter().enumerate() {
        out_str.push_str(docs);
        out_str.push_str("\nrulefile: ");
        let re = Regex::new(r".*hayabusa-rules/").unwrap();
        out_str.push_str(&re.replace(file, ""));
        if i < merged_yaml.len() - 1 {
            out_str.push('\n');
            out_str.push_str("---\n");
        }
    }
    Ok(out_str)
}

fn xor_encode(data: &str, key: u8) -> Vec<u8> {
    data.bytes().map(|b| b ^ key).collect()
}

fn xor_decode(data: &[u8], key: u8) -> Vec<u8> {
    data.iter().map(|&b| b ^ key).collect()
}

fn read_and_format_files(files: Vec<String>) -> io::Result<String> {
    let mut output = String::new();
    let re = Regex::new(r"^.*config/").unwrap();
    for file in files {
        let mut content = String::new();
        File::open(&file)?.read_to_string(&mut content)?;
        let formatted_path = re.replace(&file, "").to_string();
        output.push_str("---FILE_START---\n");
        output.push_str(&format!("path: {}\n", formatted_path));
        output.push_str("---CONTENT---\n");
        output.push_str(&content);
        output.push_str("\n---FILE_END---\n");
    }
    Ok(output)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        std::process::exit(1);
    }

    // encode
    let dir = Path::new(&args[1]);
    let yaml_files = list_files_with_extensions(dir, &["yml"])?;
    let merged_yaml = merge_yaml_files(yaml_files)?;
    let encrypted_yaml = xor_encode(&merged_yaml, 0xAA);
    let mut output_file = File::create(&args[2])?;
    output_file.write_all(&encrypted_yaml)?;

    // decode
    let encoded_file_path = Path::new(&args[2]);
    let mut encoded_file = File::open(encoded_file_path)?;
    let mut encoded_content = Vec::new();
    encoded_file.read_to_end(&mut encoded_content)?;
    let decoded_content = xor_decode(&encoded_content, 0xAA); // Example key: 0xAA
    let mut output_file = File::create("decoded_rules.yml")?;
    output_file.write_all(&decoded_content)?;

    let config_dir = dir.join("config");
    let txt_yml_files = list_files_with_extensions(&config_dir, &["yaml", "txt"])?;
    let formatted_content = read_and_format_files(txt_yml_files)?;
    let mut output_file = File::create(&args[3])?;
    output_file.write_all(formatted_content.as_bytes())?;
    Ok(())
}
