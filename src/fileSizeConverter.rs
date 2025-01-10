use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use zip::{ZipWriter, write::FileOptions, CompressionMethod, ZipArchive};
use std::io;
use zip::write::ExtendedFileOptions;

pub fn compressFile() -> Result<(), Box<dyn std::error::Error>>{
    loop{
        let mut inp = String::new();
        let mut oup = String::new();

        println!("input 0 to exit");
        println!("input file path to compress");
        io::stdin().read_line(&mut inp).expect("Failed to read line");
        if inp.trim() == "0" || oup.trim() == "0" {
            return Ok(());
        }

        println!("where is output file path");
        io::stdin().read_line(&mut oup).expect("Failed to read line");

        if inp.trim() == "0" || oup.trim() == "0" {
            return Ok(());
        }

        if let Err(e) = compress_to_zip(inp.trim(), oup.trim()) {
            eprintln!("An error occurred while compressing from '{}' to '{}': {}", inp.trim(), oup.trim(), e);
        } else {
            println!("Successfully compressed file from '{}' to '{}'", inp.trim(), oup.trim());
        }
    }
}

pub fn deCompressFile() -> Result<(), Box<dyn std::error::Error>>{
    loop{
        let mut inp = String::new();
        let mut oup = String::new();

        println!("input 0 to exit");
        println!("input file path to decompress");
        io::stdin().read_line(&mut inp).expect("Failed to read line");
        if inp.trim() == "0" || oup.trim() == "0" {
            return Ok(());
        }
        println!("enter output file path");
        io::stdin().read_line(&mut oup).expect("Failed to read line");
        if inp.trim() == "0" || oup.trim() == "0" {
            return Ok(());
        }
        if let Err(e) = decompress_file(inp.trim(), oup.trim()) {
            eprintln!("An error occurred while decompressing from '{}' to '{}': {}", inp.trim(), oup.trim(), e);
        } else {
            println!("Successfully decompressed file from '{}' to '{}'", inp.trim(), oup.trim());
        }

    }
}

fn compress_to_zip(source_file: &str, zip_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut source = File::open(source_file)?;
    let mut buffer = Vec::new();
    source.read_to_end(&mut buffer)?;

    let zip = Path::new(zip_file);
    let zip_file_handler = File::create(&zip)?;
    let mut zipped = zip::ZipWriter::new(zip_file_handler);

    let options: FileOptions<'_, ExtendedFileOptions> = FileOptions::default().compression_method(CompressionMethod::Deflated);
    zipped.start_file(Path::new(source_file).file_name().unwrap().to_string_lossy(), options)?;
    zipped.write_all(&buffer)?;
    zipped.finish()?;

    println!("Compressing file: {} to {}", source_file, zip_file);
    Ok(())
}

fn decompress_file(zip_file: &str, result_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let  source = File::open(zip_file)?;
    let mut archive = ZipArchive::new(source)?;

    for i in 0..archive.len(){
        let mut zip_file = archive.by_index(i)?;
        let output_path = format!("{}/{}", result_file, zip_file.name());

        let mut output_file = File::create(&output_path)?;
        io::copy(&mut zip_file, &mut output_file)?;
    }

    Ok(())
}