use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("---.---Guess the number!!!---.---");
	
	loop {
		println!("Enter your guess.");
		let mut guess = String::new();
		
		io::stdin()
			.read_line(&mut guess)
			.expect("Failed to read line");
			
		/* Alternative would be:
		io::stdin().read_line(&mut guess).expect("Failed to read line");
		*/
		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,  // This line can be without ,
		};
		println!("You  guessed: {guess}");
		
		println!("Thinking of a random number....");
		let secret_number = rand::thread_rng().gen_range(1..=10); //start..=end, inclusive bounds
		// This returns an u32 -> unsigned 32 bit number. 
		//println!("The secret number is: {secret_number}");
		
		match guess.cmp(&secret_number) {
			Ordering::Less => println!("Too small!"),  // arms, similar to switch case
			Ordering::Greater => println!("Too big!"),
			Ordering::Equal => { 
				println!("You win!");
				break;
			}
		}
	}

}
