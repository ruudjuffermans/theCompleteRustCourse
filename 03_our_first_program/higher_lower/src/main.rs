use rand::Rng;
use std::io;

fn main() {
    // give the user some information
    println!("welcome to the higher lower game");

    // generate a random number
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // print the random number
    println!("the secret number = {}", secret_number);

    println!("please input your guess");

    // initialize a variable
    // let counter1 = 5; // immutable
    // let mut counter2 = 5; // mutable
    let mut guess = String::new();

    // store the user provided number
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");

    // notify user of stored value
    println!("we stored your guess: {}", guess);
}
