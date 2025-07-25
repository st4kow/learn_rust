//Can store any type of coordinates
struct Point<T> {
    x: T,
    y: T
}

impl<T> Point<T> {   // impl<t> means we are implementing for the generic type
	fn x(&self) -> &T {
		&(self.x)
	}
}
impl Point<f64> {  // This function is implemented only for f64 variant
	fn distance_from_origin(&self) -> f64 {
		(self.x.powi(2) + self.y.powi(2) ).sqrt() // These are available only for floats
	}
}

struct Point_mixed<T, U> {
	x: T,
	y: U
}
impl<T, U> Point_mixed<T, U>   {  //This funtion uses different generic labels
	fn mixup<T2, U2>(self, other: Point_mixed<T2, U2>) -> Point_mixed<T, U2> {
	   Point_mixed {
		   x: self.x,
		   y: other.y
	   }
	}
}

fn main() {                  
    let number_list = vec![1, 5, 45, 100, -4];
	let result = largest_i32(&number_list);
	println!("The largest number is: {result}");
	
	let char_list = vec!['y', 'm', 'a', 'q'];
	let result = largest_char(&char_list);
	println!("The largest char is: {result}");

    //////////
	
	println!("Using a generic function for the same purpose");
	let result = largest(&number_list);
	println!("The largest number is: {result}");
	
	let result = largest(&char_list);
	println!("The largest char is: {result}");
	
	/////////
	
	//Using generics in struct definitions
	let int_point = Point {x:5, y:10};
	let float_point = Point{x:5.0, y:10.0};
	let int_point: Point<i32> = Point {x:5, y:10};
	let float_point: Point<f64> = Point {x:5.0, y:10.0};
	
	//let mixed_point = Point {x:5, y:10.0}; //Compilation error
	
	/////////
	
	let mixed_point = Point_mixed {x:5, y:10.0};
	let mixed_point = Point_mixed {x:5.0, y:10};
	let mixed_point: Point_mixed<i32, f64> =
	    Point_mixed {x:5, y:10.0};
	let mixed_point: Point_mixed<f64, i32> =
	    Point_mixed {x:5.0, y:10};
    let mixed_point: Point_mixed<char, f64> =
	    Point_mixed {x:'A', y:42.666};
		
   /////////
   //Generics in methods
   let p: Point<i32> = Point{ x:5, y:10 };
   println!("x coordinate of the Point is {}",p.x() );
	
}

//Gives reference to the largest number in a i32 slice
fn largest_i32(list: &[i32]) -> &i32 {
	let mut largest = &list[0];
	for item in list {
		if item > largest {
			largest = item;
		}
	}
	largest
}

fn largest_char(list: &[char]) -> &char {
	let mut largest = &list[0];
	for item in list {
		if item > largest {
			largest = item;
		}
	}
	largest
}

//Make it generic
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {  //T is any generic that implements ">" and "<"
	let mut largest: &T = &list[0]  ;
	for item in list {
		if item > largest {
			largest = item;
		}
	}
	largest
}