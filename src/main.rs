use std::io;

fn main() {
    println!("Guess a number!");
    println!("Input your guess:");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");
    println!("you guessed {guess}");
}
