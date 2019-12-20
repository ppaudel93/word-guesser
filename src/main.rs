use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut lives = 6;

    print!("{}[2J", 27 as char);
    println!("\u{001b}[31m# Welcome to number guesser.\n");
    println!("\u{001b}[37m# You have 6 lives.\n");
    println!("# The secret number is between 1 and 100.\n");

    loop {
        println!("Input your guess");
        println!("Lives left: {}", lives);
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse()
            .expect("Please enter a number!");

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
                lives = lives - 1;
                if lives == 0 {
                    println!("Lives are spent. You lose!");
                    println!("Secret number was {}", secret_number);
                    break;
                }
            },
            Ordering::Greater => {
                println!("Too big!");
                lives = lives - 1;
                if lives == 0 {
                    println!("Lives are spent. You lose!");
                    println!("Secret number was {}", secret_number);
                    break;
                }
            },
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }

}
