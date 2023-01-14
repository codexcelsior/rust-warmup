use std::io; //Use standard input and output
use rand::Rng; // Use standard input output
use std::cmp::Ordering; //Using Standard compare ordering

fn main() {

    println!("Guess the number!"); //print line Guess the number

    let secret_number = rand::thread_rng().gen_range(1..=100); //Generate a secret number by randoming a number

    println!("Please input your guess."); //print line Please input your guess

    let mut guess = String::new(); //create a new string named guess 

    io::stdin() 
        .read_line(&mut guess) //using read_line to pass a guess into a string
        .expect("Failed to read line"); //expect to print out Failed to read line if the number is error

    //  --snip--
    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {guess}"); 


    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }


    // let x = 5;
    // let y = 10;
    // println!("x is {x} and y + 2 = {}", y + 2);    
}