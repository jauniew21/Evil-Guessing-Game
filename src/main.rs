use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secr_num = rand::thread_rng().gen_range(100000000..=999999999);

    println!("Guess, I Dare You!");

    let digits: Vec<_> = secr_num.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();

    // for digit in digits {
    //     println!("{}", digit);
    // }

    loop {
        println!("Input your Guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("could not read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You Guessed {guess}");

        let u_digits: Vec<_> = guess.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();

        // for digit in u_digits {
        //     println!("{}", digit);
        // }

        match u_digits[u_digits.len()-1].cmp(&digits[u_digits.len()-1]) {
            Ordering::Greater => println!("Too high!"),
            Ordering::Less => println!("Too low!"),
            Ordering::Equal => {
                if guess != secr_num {
                    println!("Good guess, Keep Going! (This digit is correct)");
                } else {
                    println!("You Win!!");
                    break;
                }
            }
        }
    }
}
