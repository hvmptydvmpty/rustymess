use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=99);

    let mut tries = 0;
    loop {
        let mut guess = String::new();
        println!("Guess the number?");
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        guess = guess.trim().to_string(); // just to see how it's done
        let guess: u32 = match guess.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a number!");
                continue;
            }
        };

        tries += 1;
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win in {} tries!", tries);
                break;
            }
        }
    }
}
