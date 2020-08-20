use std::io;
use rand::Rng;

fn main(){
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1,101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    //Variables are immutable by default
    let mut guess = String::new(); //Creates a mutable variable
    //::new calls the static function new to create a new empty string

    io::stdin()
        .read_line(&mut guess) //The & is a reference similar to how C does it
        .expect("Failed to read line"); //Result has a method called expect
                                        //for what to print when the program crashes
    println!("You guessed: {}", guess); //{} means a varaible will be printed in its place
    //The Cargo.lock file keeps track of the versions of dependencies we are using
    //If we want to update cargos we use the update command

}
