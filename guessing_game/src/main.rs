use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); //gen_range is inclusive on the lower bound but exclusive on the upper bound.
    println!("The secret number is: {secret_number}");

    loop {

        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess) //The & indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times.
            .expect("Failed to read line"); //Handling Potential Failure with Result; Result is an enum that has the variants Ok and Err. The expect method on Result is used here. If the Result value is the Ok variant, expect will return the value inside the Ok. If the Result is the Err variant, expect will call the panic! macro.


        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}