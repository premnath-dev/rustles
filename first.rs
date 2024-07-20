#![allow(unused_imports, dead_code)]
use std::io::{self, Write};
fn main() {
    let a: i32 = 5;
    let b: i32 = 6;
    let c: i32 = a + b;
    println!("Hello, world!");
    println!("a: {}, b: {}, c: {}", a, b, c);

   // println!("Hello, {}!", get_name());

    let numbers = [1, 2, 3, 4, 5];
    let mut sum = 0;

    for n in 0..numbers.len() {
        sum += numbers[n];
    }
    println!("Sum: {}", sum);

    for number in numbers.iter() {
        sum += number;
    }
    println!("Sum: {}", sum);

   /*  for number in &numbers[0..2] {
        sum += *number;
    }   
    println!("Sum: {}", sum); */
    let mut values = vec![1, 2, 3];
    let mut sum = 0;

    values.push(4);
    values.push(5);

    for n in values {
        sum += n;
    }

    println!("Sum: {}", sum);
}

fn get_name() -> String {
    
    print!("What's your name? ");
    io::stdout().flush().unwrap();  // Ensure the prompt is displayed immediately

    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");
    let name = name.trim();  // Remove trailing newline

    name.to_string()
}