use std::io;

fn main(){
    println!("Guess the number!");

    println!("Please input your guess.");

    //Variables are immutable by default
    let mut guess = String::new(); //Creates a mutable variable
    //::new calls the static function new to create a new empty string

    io::stdin()
        .read_line(&mut guess) //The & is a reference similar to how C does it
        .expect("Failed to read line"); //Result has a method called expect
                                        //for what to print when the program crashes
    println!("You guessed: {}", guess); //{} means a varaible will be printed in its place
}
