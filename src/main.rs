use std::io; // Bring the input/output library into scope from the standard library (std)

fn main() {
    println!("Guessing the number!");

    println!("Please input your guess");

    let mut guess = String::new();

    io::stdin()
         .read_line(&mut guess)
         .expect("Failed to read line");
    
    println!("You guessed {}", guess);
}