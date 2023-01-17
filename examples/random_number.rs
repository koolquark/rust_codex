use std::io; 
use rand::Rng; 


fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut guess = String::new(); 


    println!("Guess a number : ");
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read input");

    println!("Secret number is : {secret_number}");
    println!("You guessed : {guess}"); 
}
