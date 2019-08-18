use std::io; // Standart input / output lib.
use rand::Rng; // Random lib -> 
               //To use this lib we need to edit and add dependency cargo.toml
use std::cmp::Ordering; // compare -> Order lib

fn main() {
    println!("Guess the number!");
    let number = rand::thread_rng().gen_range(1,50); 
    let mut lives = 5;
    while lives >= 0 
    {
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input);
        println!("Your guess is {}",user_input);
        let user_input:u32 = match user_input.trim().parse()
        {
            Ok(num) => num,
            Err(ex) =>
            {
                println!("An exception occured -> {}",ex);
                continue;
            }
        };
        
        if (lives == 0)
        {
            println!("Number is {} \nYou Lost!",number);
            break;
        }

        match user_input.cmp(&number) // match is similar to Switch-Case 
        {
            Ordering::Less => println!("{} is too small \nTry Again! ({} lives left)",user_input,lives),
            Ordering::Greater => println!("{} is too big \nTry Again! ({} lives left)",user_input,lives),
            Ordering::Equal =>
            {
                println!("You win");
                break;
            }
        }
        lives -= 1;
    }
}
