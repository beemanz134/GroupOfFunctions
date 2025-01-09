use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

pub fn read_txt() -> io::Result<Vec<String>>{
    // Create a PathBuf that goes up one level and then into the storage directory
    let fpath: PathBuf = Path::new("storage/ToDo.txt").to_path_buf();

    if !fpath.exists(){
        println!("File does not exist: {:?}", fpath);
    }

    let file = File::open(fpath)?;
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines()
        .filter_map(Result::ok) // Filter out any errors
        .collect();
    Ok(lines)
}

pub fn showList() -> io::Result<()> {
    let items = read_txt()?;

    for (index, item) in items.iter().enumerate() {
        println!("{}: {}", index + 1, item); // Add line number here
    }
    Ok(())
}

pub fn addItem()-> io::Result<()>{
    let mut input = String::new();
    println!("what would you like to add?");
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let fpath: PathBuf = Path::new("storage/ToDo.txt").to_path_buf();

    if !fpath.exists() {
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
pub fn updateItem()-> io::Result<()> {
    let mut newItem = String::new();
    let mut line = String::new();
    println!("what line would you like to change");
    io::stdin().read_line(&mut line).expect("Failed to read line");
    let line_num: usize = line.trim().parse().expect("Please enter a valid number");
    println!("enter new item");
    io::stdin().read_line(&mut newItem).expect("Failed to read line");

    let fpath: PathBuf = Path::new("storage/ToDo.txt").to_path_buf();
    let mut items = read_txt()?;

    if line_num > 0 && line_num < items.len() {
        items[line_num - 1] = newItem.trim().to_string();
    } else {
        println!("line invalid");
        return Ok(());
    }

    let mut file = OpenOptions::new().write(true).truncate(true).open(&fpath)?;
    for item in items{
        writeln!(file, "{}", item)?;
    }
    Ok(())
}

pub fn deleteItem()-> io::Result<()>{
    let mut line = String::new();
    println!("what line would you like to delete?");
    io::stdin().read_line(&mut line).expect("Failed to read line");
    let line_num: usize = line.trim().parse::<usize>().expect("Please enter a valid number") - 1;

    let fpath: PathBuf = Path::new("storage/ToDo.txt").to_path_buf();
    let items = read_txt()?;
    if line_num >= items.len() {
        println!("Line number {} is out of bounds. Please enter a number between 1 and {}.", line_num, items.len() - 1);
        return Ok(());
    }

    let read_lines: Vec<String> = items.into_iter()
        .enumerate()
        .filter_map(|(index, item)| {
            if index != line_num {
                Some(item)
            } else {
                None
            }
        })
        .collect();

    let mut file = OpenOptions::new().write(true).truncate(true).open(&fpath)?;
    for item in read_lines {
        writeln!(file, "{}", item)?;
    }
    Ok(())
}