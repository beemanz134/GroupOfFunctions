use age::{Encryptor, Decryptor};
use secrecy::SecretString;
use std::{fs, io};
use std::io::{Write, Read};
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

pub fn file_decrypt()  -> Result<(), Box<dyn std::error::Error>>{
    let mut input = String::new();
    let mut output = String::new();
    let mut key = String::new();

    println!("Enter file to decrypt:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input = input.trim_matches('"').trim().to_string();

    println!("Enter the secret code:");
    io::stdin().read_line(&mut key).expect("Failed to read line");
    key = key.trim().to_string();

    println!("Enter destination to output decrypted file:");
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

    if let Err(e) = decrypt(&input, encrypted_file_path.to_str().unwrap(), &key) {
        eprintln!("An error occurred while decrypting '{}' because '{}'", input, e);
        return Err(e);
    } else {
        println!("Successfully decrypted file to '{}'", encrypted_file_path.display());
        Ok(())
    }

}

fn decrypt(source_file: &str, output: &str, key: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Read the encrypted data from the source file
    let data = fs::read(source_file)?;

    // Create a SecretString from the provided key
    let passphrase = SecretString::from(key.to_owned());

    // Create a decryptor using the passphrase
    let decryptor = Decryptor::new(passphrase)?;

    // Create a reader for the encrypted data
    let encrypted_reader = &data[..]; // Create a slice of the encrypted data

    // Attempt to decrypt the data
    let mut decrypted_data = decryptor.decrypt(encrypted_reader)?;

    // Create the output file
    println!("Attempting to create output file at: {}", output);
    let mut output_file = fs::File::create(output)?;

    // Read from the StreamReader and write to the output file
    let mut buffer = Vec::new();
    decrypted_data.read_to_end(&mut buffer)?; // Read all decrypted data into a buffer
    output_file.write_all(&buffer)?; // Write the buffer to the output file

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