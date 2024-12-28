use std::io;
use random_number::random;

pub fn start(){
    let randomGuess: u8 = random!(0..255);
    println!("Guess a number between 0 and 255!");

    loop{
        println!("input your guess computer guess -> {}", randomGuess);
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        match guess.trim().parse::<u32>() {
            Ok(guess) => {
                // Check if the guess is within the valid range
                if guess > 255 {
                    println!("Please enter a number between 0 and 255.");
                    continue;
                }

                // Compare the guess with the random number
                if randomGuess as u32 == guess {
                    println!("Congratulations! You guessed the correct number: {}", randomGuess);
                    break;
                } else {
                    println!("Wrong guess, try again.");
                }
            }
            Err(_) => {
                println!("Invalid input, please enter a valid number.");
            }
        }
    }

}