use std::io;
use std::cmp::Ordering;
use rand::Rng;
//Cargo.lock file is really important for the reproducibility of the code with the same dependecies as is for the first build
//Remains same until manually upgraded to the latest dependecy

fn main()
{
    //generate a secret number to be compared with using the random library
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Guess the Number: ");
        //take the input from the user on the terminal 
        let mut guess = String::new();
        //take the input from the user on the command line and convert into u32
        io::stdin().read_line(&mut guess)
            .expect("Cant read the number");

        //not trim the guess
        //this syntax will continue on error instead of just simply crashing. Beautifull
        let guess: u32 =match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        println!("Guessed Number {guess}");
        //Now compare the guesseed number and the secret number using the standar compare
        //and match
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Guess is too Low"),
            Ordering::Greater => println!("Guess is too high"),
            Ordering::Equal => {
                println!("You guesssed it right! Congratulation <3.");
                break;
            }
        }
    }
}