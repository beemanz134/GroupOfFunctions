use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use zip::{ZipWriter, write::FileOptions, CompressionMethod};
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
    Ok(())
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
