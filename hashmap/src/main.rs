use std::collections::HashMap;


fn main() {
	let mut scores: HashMap<String, i32> = HashMap::new();
	scores.insert(String::from("Blue"), 10);
	scores.insert(String::from("Yellow"), 50);
	
	//Accessing values in the hash map
	
	let team_name = String::from("Blue");
	let score = scores.get(&team_name)
	                 .copied()
					 .unwrap_or(0);
    print_team(&team_name, &score);
	
	println!("Iterating through the HashMap...");
	for (key, value) in &scores {
		print_team(&key, &value);
	}
	
	// HashMap takes ownership of owned types.
	let field_name = String::from("Favorite color");
	let field_value = String::from("Blue");
	let mut map: HashMap<String, String> = HashMap::new();
	map.insert(field_name, field_value);
	// field_name and field_value are unuseable from now
	// println!("{field_name},{field_value}"); Test line to gen error
	
	//Overwriting values
	let mut scores: HashMap<String, i32> = HashMap::new();
	scores.insert(String::from("Blue"), 10);
	println!("{scores:?}");
	scores.insert(String::from("Blue"), 20); // This key exist, rust overrides
	println!("{scores:?}"); 
	
	//Using entry. Returns enum to check if it already exists
	scores.entry(String::from("Yellow")).or_insert(50);
	scores.entry(String::from("Blue")).or_insert(50); // Means add Blue-50 if key does not exists
	// Entry adds a key if it is not there already with non existing value
	// or_insert Add value if it was a new key
	println!("{scores:?}");
	
	//Updating a value based on an old value
	let text = "hello world wonderful world";
	let mut map: HashMap<&str, i32> = HashMap::new();
	for word in text.split_whitespace() {
		let count = map.entry(word).or_insert(0);
		(*count) += 1; //or_insert gives back a mutable reference to the stored value, that is sotred in count
	}
	println!("{map:?}");
}
fn print_team(team: &String, score: &i32) {
	println!("Team: {team}, Score: {score}");
}
