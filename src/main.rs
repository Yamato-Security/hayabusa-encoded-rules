use std::env;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;
use yaml_rust::{YamlEmitter, YamlLoader};

fn list_yaml_files(dir: &Path) -> io::Result<Vec<String>> {
    let mut yaml_files = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            yaml_files.extend(list_yaml_files(&path)?);
        } else if path.extension().and_then(|s| s.to_str()) == Some("yml") {
            yaml_files.push(path.to_string_lossy().to_string());
        }
    }
    Ok(yaml_files)
}

fn merge_yaml_files(files: Vec<String>) -> Result<String, Box<dyn std::error::Error>> {
    let mut merged_yaml = Vec::new();
    for file in files {
        let mut content = String::new();
        File::open(&file)?.read_to_string(&mut content)?;
        let docs = YamlLoader::load_from_str(&content)?;
        merged_yaml.extend(docs);
    }
    let mut out_str = String::new();
    for (i, doc) in merged_yaml.iter().enumerate() {
        if i > 0 {
            out_str.push('\n'); // Add separator between documents
        }
        {
            let mut emitter = YamlEmitter::new(&mut out_str);
            emitter.dump(doc)?;
        }
    }
    Ok(out_str)
}

fn xor_encrypt(data: &str, key: u8) -> Vec<u8> {
    data.bytes().map(|b| b ^ key).collect()
}

// fn xor_decrypt(data: &[u8], key: u8) -> Vec<u8> {
//     data.iter().map(|&b| b ^ key).collect()
// }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <directory>", args[0]);
        std::process::exit(1);
    }
    let current_dir = env::current_dir()?;
    let dir = current_dir.join(&args[1]);
    let yaml_files = list_yaml_files(&dir)?;
    let merged_yaml = merge_yaml_files(yaml_files)?;
    println!("***\n {}", merged_yaml);
    let encrypted_yaml = xor_encrypt(&merged_yaml, 0xAA);
    println!("@@@\n {}", merged_yaml);
    let mut output_file = File::create(&args[2])?;
    output_file.write_all(&encrypted_yaml)?;

    // // decrypt
    // let encrypted_file_path = Path::new("encrypted_output.yml");
    // let mut encrypted_file = File::open(encrypted_file_path)?;
    // let mut encrypted_content = Vec::new();
    // encrypted_file.read_to_end(&mut encrypted_content)?;
    // let decrypted_content = xor_decrypt(&encrypted_content, 0xAA); // Example key: 0xAA
    // let mut output_file = File::create("decrypted_output.yml")?;
    // output_file.write_all(&decrypted_content.as_bytes())?;
    Ok(())
}
