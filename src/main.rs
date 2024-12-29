mod tempConverter;
mod guessGame;
mod list_worker;

use std::io;
use std::env;

fn main() {
    loop {
        let mut input = String::new();
        println!("press 1 for temperature converter ");
        println!("press 2 for guessing game ");
        println!("press 3 for a todo list ");
        println!("press 4 for compress decompression file tool ");
        println!("press 5 for read and write to json ");
        println!("press 6 for a web crawler ");
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
            4 => println!("4"),
            5 => println!("5"),
            6 => println!("6"),
            7 => println!("7"),
            8 => println!("8"),
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
            3 => list_worker::updateItem(),
            4 => list_worker::deleteItem(), //delete by item number
            0 => break,
            _ => println!("unrecognized value"),
        }
    }
}