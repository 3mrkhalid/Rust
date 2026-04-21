use std::io;

fn main() {

    println!("!Guess the number!");

    println!("please Enter your number");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Faild to read line!");

    println!("You guess: {guess}");
}