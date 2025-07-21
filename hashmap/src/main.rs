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
	let field_value = String::from("Blue")
	let mut map: HashMap<String String> = HashMap::new();
	// TODO
	
}
fn print_team(team: &String, score: &i32) {
	println!("Team: {team}, Score: {score}");
}
