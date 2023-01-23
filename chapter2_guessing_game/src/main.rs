use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main()
{
    //generate a secret number random\
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    loop{
        //get the number from the user
        println!("Guess the number mf: ");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Can't read the number bro.");
        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        //now we have the number match it using the cmp
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("To0 Low!"),
            Ordering::Greater => println!("Too High"),
            Ordering::Equal => {
                println!("Congratulations! You Win.");
                break;
            }
        }

    }
}
