use rand::Rng;
use std::cmp::Ordering;
use std::{fmt, io};

pub struct Guess {
    value: i32,
}

struct InvalidGuessError {
    message: String,
}

impl Guess {
    pub fn new(value: i32) -> Result<Guess, InvalidGuessError> {
        if value < 1 {
            return Err(InvalidGuessError {
                message: "Guess value was way to low. It must be greater than 1.".to_string(),
            });
        } else if value > 100 {
            return Err(InvalidGuessError {
                message: "Guess value was to way to high. It must be less than 100.".to_string(),
            });
        }

        Ok(Guess { value })
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

impl fmt::Display for Guess {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Please input your guess.");

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: Guess = match guess.trim().parse() {
            Ok(num) => match Guess::new(num) {
                Ok(num) => num,
                Err(error) => {
                    println!("{}", error.message);
                    continue;
                }
            },
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.value.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
