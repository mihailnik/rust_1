use std::io;

fn main() {
    println!("Guess the nummer!");
    println!("Please input your guess: ");
    let mut guess: String = String::new();
    let mut guess_input_not_valid: String = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect(&mut guess_input_not_valid);
    print!("You guessed: {}", guess);
}
