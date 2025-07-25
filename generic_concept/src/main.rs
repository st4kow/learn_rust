fn main() {
    // Concept of generalization
	let number_list = vec![12, 32, 54, 76, 100, 32];
	let mut largest_num = &number_list[0];
	for number in &number_list {
		if number > largest_num {
			largest_num = number;
		}
	}
	
    println!("The largest number is: {largest_num}");

    //After moving it to a function
	println!("Using functions: ");
	let result: &i32 = largest(&number_list);
	println!("The largest number is: {result}");
			  s
	let number_list = vec![243, 652, -2345, 32];
	let result: &i32 = largest(&number_list);
	println!("The largest number is: {result}");

}

fn largest(list: &[i32]) -> &i32 {
	let mut largest: &i32 = &list[0];
	for item in list {
		if item > largest {
			largest = item;
		}
	}
	largest
}
