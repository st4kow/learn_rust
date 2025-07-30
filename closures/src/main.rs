use std::thread;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
	Red,
	Blue
}
struct Inventory {
	shirts: Vec<ShirtColor>
}
impl Inventory {
	fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
		user_preference.unwrap_or_else(|| self.most_stocked() )
	}
	fn most_stocked(&self) -> ShirtColor {
		let mut num_red = 0;
		let mut num_blue = 0;
		
		for color in &self.shirts {
			match color {
				ShirtColor::Red => num_red += 1,
				ShirtColor::Blue => num_blue += 1
			}
		}
		if num_red > num_blue {
			ShirtColor::Red
		} else {
			ShirtColor::Blue
		}
	}
}

#[derive(Debug)]
struct Rectange {
	width: u32,
	height: u32,
}
fn main() {
    let store = Inventory {
		shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue]
	};
	let user_pref1: Option<ShirtColor> = Some(ShirtColor::Red);
	let giveaway1: ShirtColor = store.giveaway(user_pref1);
	println!("The user with preference {:?} gets {:?}", user_pref1, giveaway1);
	
	let user_pref2: Option<ShirtColor> = None;
	let giveaway2: ShirtColor = store.giveaway(user_pref2);
	println!("The user with preference {:?} gets {:?}", user_pref2, giveaway2);
	
	//////////////////////////
	// Storing a closure in a variable, also annotating types
	let two: u32 = 2;
	let _expensive_closure = |num: u32| -> u32 {
		println!("calculating slowly...");
		num + two //different from a function! I could access the two variable this way
	};

	//////////////////////////
	// Testing different behavior clsure can do
	// 1, borrowing immutably
	// 2, borrowing mutably
	// 3, taking ownership
	// The exact action is autmatically decied based on what the body does

	println!("----Testing different closure behaviors----");

	println!("**Only borrows:");
	let list: Vec<i32> = vec![1, 2, 3];
	println!("Before defining closure: {list:?}");
	let only_borrows = || println!("From closure: {list:?}");
	println!("Before calling closure: {list:?}");
    only_borrows();
	println!("After after calling closure: {list:?}");

	println!("**Mutably borrows:");
	let mut list: Vec<i32> = vec![1, 2, 3];
	println!("Before defining closure: {list:?}");
	let mut borrows_mutably = || list.push(7);
	//println!("Before calling closure: {list:?}"); // CANNOT DO THIS because of the mutable refernce of closure
    borrows_mutably();
	println!("After after calling closure: {list:?}");

	//move command can be used to force taking ownership
	let list: Vec<i32> = vec![1, 2, 3];
	println!("Before defining closure: {list:?}");
	thread::spawn(move || println!("From thread: {list:?}")) //Forcing the closure to get ownership of list instead of inmout borrow
		.join()
		.unwrap();
	//println!("After after calling closure: {list:?}"); //Cannot do this, does not exist anymore

	// MOVING CAPTURED VALUES OUT OF THE CLOSURE, FN TRAITS
	println!("----Moving captured values out of the closure----");

	// FnOnce - these closures can be called once. All closures implement this
	        // Closures that moves captured values out of there body implements only this
	// FnMut  - For catpures that do not move captured values out of their body, but might mutate
	        // captured values. Can be called more than once
	// Fn     - Applies for closured that do not mutate and do not move capture values out of there body.
	        // can be called more tha nonce
    // Testing Fn traits..

	let mut list: [Rectange; 3] = [
		Rectange { width: 10, height: 1},
		Rectange { width: 3,  height: 5},
		Rectange { width: 7, height: 12}
	];
	let mut cnt: u32 = 0u32;
	list.sort_by_key(|r: &Rectange| {
		cnt += 1; //This works for cnt, because cnt exists in every iterations
		r.width
	});
	println!("{list:#?}");
	println!("Num of operations: {cnt}");


}
 
