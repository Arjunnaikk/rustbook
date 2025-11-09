use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn main() {
    println!("Guess the number !!");
    let secret = rand::rng().random_range(1..=100);
    // println!("{secret}");
    loop {
        println!("Please enter your guess");

        let mut g = String::new();
        io::stdin().read_line(&mut g).expect("Failed to read line");
        println!("Your guess is {g}");

        let g: u32 = match g.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter an integer");
                continue;
            },
        };
        match g.cmp(&secret) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You got it!");
                break;
            }
        }
    }
}
