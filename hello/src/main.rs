use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    let mut range: i32 = 32;
    range = range * 2;
    let (a, b): (i32, i32) = (0, range);
    println!("a: {}, b: {}", a, b);
    println!("Hello, world!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Please input your guess.");
    let secret_number = secret_number.to_string();
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("The secret number is: {}", secret_number);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}

/*
let guess: u32 = guess
    .trim()
    .parse()
    .expect("Please type a number!");

let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};

--------------------
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

*/