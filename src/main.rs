mod tempConverter;
mod guessGame;
mod list_worker;
mod fileSizeConverter;
mod jsonWorker;
mod do_webcrawler;
mod fileSecret;
mod aServer;

use std::io;

use std::env;

fn main() {
    loop {
        let mut input = String::new();
        println!("press 1 for temperature converter ");
        println!("press 2 for guessing game ");
        println!("press 3 for a todo list ");
        println!("press 4 for compress decompression file tool ");
        println!("press 5 for read and write to json *******CANCLLED************"); //*******CANCLLED************ not sure scope or usecase
        println!("press 6 for a web crawler (note: only grabs a links can be altered to grab other things) ");
        println!("press 7 for file encryption/decryption ");
        println!("press 8 for a http server ");
        println!("press 0 to exit");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let num: u8 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number");
                continue; // Go back to the start of the loop
            }
        };

        match num {
            1 => temperature_converter_controller(), // No need to check for exit
            2 => guess_controller(),
            3 => list_controller(),
            4 => file_size_converter(),
            5 => println!("5"), //cancelled json converter
            6 => webcrawler(),
            7 => fileSecret(),
            8 => startServer(),
            0 => {
                println!("Exiting the program");
                break; // Breaks out of the outer loop
            }
            _ => println!("unrecognized value"),
        }
    }
}

fn temperature_converter_controller() {
    loop {
        let mut input = String::new();
        println!("press 1 for fahrenheit -> celsius");
        println!("press 2 for celsius -> fahrenheit");
        println!("press 0 to return to main menu");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let num: u8 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number");
                continue; // Go back to the start of the inner loop
            }
        };

        match num {
            1 => tempConverter::convertFtC(),
            2 => tempConverter::converCtF(),
            0 => break, // Just break out of the inner loop
            _ => println!("unrecognized value"),
        }
    }
}
fn guess_controller() {
    loop {
        let mut input = String::new();
        println!("press 1 to start");
        println!("press 0 to return to main menu");
        io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

        let num: u8 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("number not recognized");
                continue;
            }
        };

        match num{
            1 => guessGame::start(),
            0 => break,
            _ => println!("unrecognized value"),
        }
    };

}

fn list_controller() {
    loop {
        let mut input = String::new();
        println!("press 1 to show list");
        println!("press 2 to add new item to  list");
        println!("press 3 to update item in list");
        println!("press 4 to delete item in list");
        println!("press 0 to return to main menu");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let num: u8 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Number not recognized");
                continue;
            }
        };

        match num {
            1 => if let Err(e) = list_worker::showList(){
                eprint!("error showing list: {}", e);
            },
            2 => if let Err(e) = list_worker::addItem(){
                eprint!("error showing list: {}", e);
            },
            3 => if let Err(e) = list_worker::updateItem(){
                eprint!("error showing list: {}", e);
            },
            4 => if let Err(e) = list_worker::deleteItem(){ //delete by item number
                eprint!("error showing list: {}", e);
            },
            0 => break,
            _ => println!("unrecognized value"),
        }
    }
}

fn file_size_converter() {
    loop {
        let mut input = String::new();
        println!("press 1 to compress");
        println!("press 2 to decompress");
        println!("press 0 to return to main menu");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let num: u8 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Number not recognized");
                continue;
            }
        };

        match num {
            1 => {
                match fileSizeConverter::compressFile() {
                    Ok(_) => println!("Successfully compressed"),
                    Err(e) => eprintln!("Error compressing: {}", e),
                }
            },
            2 => {
                match fileSizeConverter::deCompressFile() {
                    Ok(_) => println!("Successfully uncompressed"),
                    Err(e) => eprintln!("Error decompressing: {}", e),
                }
            },
            0 => break,
            _ => println!("Unrecognized value"),
        }
    }
}

pub fn json_converter() {
    loop {
        let mut input = String::new();
        println!("press 1 to serialize to json");
        println!("press 2 to deserialize");
        println!("press 0 to exit");

        io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
        let num: u8 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Number not recognized");
                continue;
            }
        };

        match num {
            1 => jsonWorker::to_json(),
            2 => jsonWorker::from_json(),
            0 => break,
            _ => println!("Unrecognized value"),
        }
    }
}

pub fn webcrawler() {
    do_webcrawler::start();
}

pub fn fileSecret() {
    loop {
        let mut input = String::new();
        println!("press 1 to encrypt a file");
        println!("press 2 to decrypt a file");
        println!("press 0 to return to main menu");
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let num: u8 = match input.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("failed num input");
                continue;
            }
        };

        match num{
            1 => {
                match fileSecret::file_encrypt()
                {
                    Ok(_) => println!("Successfully encrypted"),
                    Err(e) => eprintln!("Error encrypting: {}", e),
                }
            },
            2 => {
                match fileSecret::file_decrypt()
                {
                    Ok(_) => println!("Successfully encrypted"),
                    Err(e) => eprintln!("Error decrypting: {}", e),
                }
            },
            0 => break,
            _ => println!("invalid entry")
        }
    }
}

pub fn startServer() {
    aServer::start();
}
