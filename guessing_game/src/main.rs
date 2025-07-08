use std::io;

fn main() {
    println!("Guess the number!!!");
	println!("Enter your guess.");
	let mut guess = String::new();
	
	io::stdin()
		.read_line(&mut guess)
		.expect("Failed to read line");
		
	/* Alternative would be:
	  io::stdin().read_line(&mut guess).expect("Failed to read line");
	*/
	
	println!("You  guessed: {guess}");
	
	let x = 123;
	let y = 321;
	println!("TEST: x is: {x}, y is: {y}, how did I get: {}?",2*x+3*y);
}
