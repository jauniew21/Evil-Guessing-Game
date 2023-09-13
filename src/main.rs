use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    // Setting up the recurring counts and the secret number
    let mut guess_count = 0;
    let mut digit_count = 0;
    let secr_num = rand::thread_rng().gen_range(100000000..=999999999);

    println!("Guess, I Dare You!");

    // Putting the individual digits of the secret number into a vector of characters
    // This makes it easier to compare individual digits later
    let digits: Vec<_> = secr_num.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();

    // for digit in digits {
    //     println!("{}", digit);
    // }

    // 'Gameplay' loop
    loop {
        println!("Input your Guess.");

        let mut guess = String::new();
        guess_count += 1;

        // Reading the user guess
        io::stdin()
            .read_line(&mut guess)
            .expect("could not read line");

        // Turning that guess into an int
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You Guessed {guess}. Guess #{guess_count}");

        // Turning that guess into a vector of characters
        let mut u_digits: Vec<_> = guess.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();

        // A safeguard for the user input breaking a u32
        while u_digits.len() > 8 {
            u_digits.pop();
        }

        // Comparing digits of user number and secret number
        for cur_digit in 0..u_digits.len() {
            if u_digits[cur_digit] == digits[cur_digit] {
                digit_count += 1;
            }
        }

        // for digit in u_digits {
        //     println!("{}", digit);
        // }

        // This match checks the last digit of the user's guess
        // it is then compared to the corresponding digit of the secret number
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
        
        print!("{digit_count} digits correct! ");
        digit_count = 0;
    }
}
