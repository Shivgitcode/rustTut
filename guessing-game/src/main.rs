use rand::Rng;
use std::io;

fn main() {
    let mut guess = String::from("");
    let randomNum = rand::thread_rng().gen_range(1..=100);

    print!("Enter a Num: ");
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("{} guessed num", guess);
    println!("{} random num", randomNum)
}
