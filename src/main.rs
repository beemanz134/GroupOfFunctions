use std::io;

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
        println!("press 9 to exit");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let num: u8 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number");
                return;
            }
        };

        match num {
            1 => println!("1"),
            2 => println!("2"),
            3 => println!("3"),
            4 => println!("4"),
            5 => println!("5"),
            6 => println!("6"),
            7 => println!("7"),
            8 => println!("8"),
            9 => {
                println!("breaking loop");
                break;
            }
            _ => println!("unrecognized value"),
        }
    }
}
