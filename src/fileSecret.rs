use age::{Encryptor, Decryptor};
use secrecy::SecretString;
use std::{fs, io};
use std::io::Write;
use std::path;


// just input path ex: "c/user/asd/fileORfolder" and output destination

pub fn file_encrypt() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    let mut output = String::new();
    let mut key = String::new();

    println!("Enter file to encrypt:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input = input.trim_matches('"').trim().to_string();

    println!("Enter a secret code to use as passphrase:");
    io::stdin().read_line(&mut key).expect("Failed to read line");
    key = key.trim().to_string();

    println!("Enter destination to output encrypted file:");
    io::stdin().read_line(&mut output).expect("Failed to read line");
    output = output.trim_matches('"').trim().to_string();

    if let Err(e) = fs::metadata(&input) {
        eprintln!("Error accessing input file '{}': {}", input, e);
        return Err(Box::new(e));
    }

    let output_path = path::Path::new(&output);
    if let Some(parent) = output_path.parent() {
        if !parent.exists() || !parent.is_dir() {
            eprintln!("Output directory '{}' does not exist or is not a directory.", parent.display());
            return Err(Box::new(io::Error::new(io::ErrorKind::NotFound, "Output directory not found")));
        }
    }

    let input_file_name = path::Path::new(&input).file_name().unwrap().to_str().unwrap();
    let encrypted_file_name = format!("{}.age", input_file_name);
    let encrypted_file_path = output_path.join(encrypted_file_name);

    if let Err(e) = encrypt(&input, encrypted_file_path.to_str().unwrap(), &key) {
        eprintln!("An error occurred while encrypting '{}' because '{}'", input, e);
        return Err(e);
    } else {
        println!("Successfully encrypted file to '{}'", encrypted_file_path.display());
        Ok(())
    }
}

pub fn file_decrypt() {
    Ok::<(), Box<dyn std::error::Error>>(());
}

fn decrypt(source_file: &str, output: &str, key: &str) -> Result<(), Box<dyn std::error::Error>>{
    Ok(())
}


fn encrypt(source_file: &str, output: &str, key: &str) -> Result<(), Box<dyn std::error::Error>> {
    let data = fs::read(source_file)?;
    let passphrase = SecretString::from(key.to_owned());
    let encryptor = Encryptor::with_user_passphrase(passphrase);

    println!("Attempting to create output file at: {}", output);

    let output_file = fs::File::create(output)?;
    let mut writer = encryptor.wrap_output(output_file)?;
    writer.write_all(&data)?;
    writer.finish()?; // Finalize the encryption

    Ok(())
}