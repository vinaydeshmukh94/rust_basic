use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    

    

    let secret_num = rand::thread_rng().gen_range(1..=100);
    // let secret_num = Rng.thread_rng().gen_range(1..=100);
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        println!("Generated secret number {secret_num}");

        io::stdin()
            .read_line(&mut guess)      // Appends the input does not overwrite guess
            .expect("Failed to read line");

        // let guess : u32 = guess.trim().parse().expect("Failed to parse"); // trim is used because extra /n is there in input
        
        let guess : u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid input");
                continue
            },
        };
        
        // println!("CURRENT : {}", guess_num);



        // io::stdin().read_line(&mut guess) returns a Result on result we are using expect 

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_num) {
            Ordering::Equal => {
                println!("You Won");
                break;
            },
            Ordering::Greater => println!("Too Big"),
            Ordering::Less => println!("Too Small")
        }
    }
}