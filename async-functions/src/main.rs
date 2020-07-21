mod factorial;
use futures::executor::block_on;
use std::io;

fn main() {
    println!("Which number does it take to calculate the factorial?");
    let mut user_inp = String::new();
    let mut user_inp_u128: u128 = 0;

    match io::stdin().read_line(&mut user_inp) {
        Ok(_n) => {
            user_inp_u128 = match user_inp.trim().parse() {
                Ok(number) => number,
                Err(ex) => {
                    println!("Exception Message: {}", ex);
                    0
                }
            };
        }
        Err(error) => println!("error: {}", error),
    }

    let result = factorial::calculate(user_inp_u128);
    let res = block_on(result);
    println!("Result is : {:?}", res);
}
