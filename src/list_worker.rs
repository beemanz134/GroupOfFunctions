use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

pub fn read_txt() -> io::Result<Vec<String>>{
    // Create a PathBuf that goes up one level and then into the storage directory
    let fpath: PathBuf = Path::new("storage/ToDo.txt").to_path_buf();

    if fpath.exists() {
        println!("File exists: {:?}", fpath);
    } else {
        println!("File does not exist: {:?}", fpath);
    }

    let file = File::open(fpath)?;
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines()
        .filter_map(Result::ok) // Filter out any errors
        .collect();
    Ok(lines)
}

pub fn showList() -> io::Result<()>{
    let items = read_txt()?;

    for item in items {
        println!("line {}", item);
    }
    Ok(())
}

pub fn addItem()-> io::Result<()>{
    let mut input = String::new();
    println!("what would you like to add?");
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let fpath: PathBuf = Path::new("storage/ToDo.txt").to_path_buf();

    if fpath.exists() {} else {
        println!("File does not exist: {:?}", fpath);
    }
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(&fpath)?;
    file.write_all(format!("{}\n", input.trim()).as_bytes())?;

    Ok(())
}
pub fn updateItem(){}

pub fn deleteItem(){}