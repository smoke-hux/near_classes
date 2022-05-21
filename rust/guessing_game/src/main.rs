use std::io;  // if you want to use this prelude you have to bring the scope of every program .
// it includes all the standard library 

fn main() {/* the fn declares a new function 

    the paranthesis , () , indidcate ther eare no parameters, ande
     curly bracket, {} ,start the body of the function
    */
    println!("Guess the number!");

    println!("Please input you guess.");

    let mut guess = String::new();// introduces mutable variable named guess and the = sign is the value that guess is bount to 

    io::stdin()
        .read_line(&mut guess)//calls the read-line methond on the standard input hand to get input from the user
        .expect("Failed to read line");

    println("You guessed: {}", guess);
}
