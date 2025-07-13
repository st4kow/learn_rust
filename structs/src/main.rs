struct User {
	active: bool,
	username: String,
	email: String,
	sign_in_count: u64
}
fn print_user_data (user: &User ) {
	println!("Data of user1: active:{}, username:{}, email:{}, sign in count: {}.",
	user.active,
	user.username,
	user.email,
	user.sign_in_count);
}

fn build_user(email: String, username: String) -> User {
    /*
	User {
		active: true,
		username: username,
		email: email,
		sign_in_count: 1
	}
	*/
	
	//This was long, we can use the "Field init Shorthand" instead
	User {
		active: true,
		username,
		email,
		sign_in_count: 1
	}
}

//Tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

//Unit-Like structs. Have no fileds, behave like ()
/*
can be useful when you need to implement a trait on some type but don’t have any data that you want to store in the type itself.
*/
struct AlwaysEqual;

fn main() {
    let mut user1: User = User {
		active: true,
		username: String::from("someusername123"),
		email: String::from("someone@example.com"),
		sign_in_count: 1
	};
	print_user_data(&user1 );
	
	user1.active = false;
	user1.username = String::from("newsomeusername123");
	
	print_user_data(&user1 );
	
	let dmyname = String::from("aname");
	let dmyemail = String::from("aname@toto.com");
	let built_user = build_user(dmyname, dmyemail);
	print_user_data(&built_user );
	
	println!("Creating instances from Other instances with struct update syntax: ");
	
	let user2 = User {
		email: String::from("abada@ed.com"),
		..user1 // Everything else is coming from this struct
	};
	print_user_data(&user2 );
	
	/*
	println!("Printing user date of user1 again: ");
	print_user_data(&user1 );
	  !!IMPORTANT!!
	  This does not work, because the username was moved into user2
	  It we would have created username by hand, both users would be valid
	  This can be fixed, by implementing Copy trait
	*/
	
	println!("--Using Tuple Structs Without Named Fields to Create DIfferent Types--");
	//Tuple structs
	
	/*
	Tuple structs are useful when you want to give the whole tuple a name and make the tuple a different type from other tuples, and when naming each field as in a regular struct would be verbose or redundant.
	*/
	let black = Color(0, 0, 0);
	let origin = Point(0, 0, 0);
	
	//destructuring
	let Point(x, y, z) = origin;
	println!("The origin is: {x},{y},{z}.");
	
	println!("--Unit-Like Structs Without Any Fields--");
	let ae: AlwaysEqual = AlwaysEqual;
	}
