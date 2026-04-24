use std::io;

fn main() {
    println!("guess the number");
    println!("Please input your guess.");

    let mut guess: String = String ::new();
    io::stdin().read_line(&mut guess);
 
    println!("You guessed: {guess}");

}

