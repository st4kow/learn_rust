pub trait Summary {                //Trait CamelCase
	fn summarize(&self) -> String; //Prototype of funtion that needs to be implemented
}
pub struct NewsArticle {
	pub headline: String,
	pub location: String,
	pub author:   String,
	pub content:  String
}
impl Summary for NewsArticle {  // Implementing Trait for NewArticle
    fn summarize(&self) -> String {
		format!("{}, by {} ({})", self.headline, self.author, self.location)
	}
}
pub struct SocialPost {
	pub username: String,
	pub content:  String,
	pub reply:    bool,
	pub repost:   bool
}
impl Summary for SocialPost { // Implementing Trait for SocialPost
    fn summarize(&self) -> String {
		format!("{}: {}", self.username, self.content)
	}
}

// Funcions

pub fn notify(item: &impl Summary) -> () { // Any item that implements Summary
	println!("Breaking News! {}", item.summarize() );
}

pub fn notify_1<T: Summary>(item: &T) -> () { // Trait bound syntax, equivalent to the previous, can express more complexity
    println!("Breaking News! {}", item.summarize() );
}
pub fn notify_2(item1: &impl Summary, item2: &impl Summary) -> () {} //Two items that implement Summary
pub fn notify_3<T: Summary>(item1: &T, item2: &T) -> () {}// Also forcing the same type
pub fn notify_4(item: &(impl Summary + Clone) ) -> () {} // Item that implements both Summary and Display
pub fn notify_5<T, U> (t: &T, u: &U) -> ()  // Alternative it it would be too long and complex
where
    T: Clone,
	U: Clone + Summary
{}

//Function that return a type that implements a Trait without naming the concrete type
// IMPORTANT: THis only works if the return value is known in compilation time
// For example if .. return A else return B does not work.
pub fn returns_summarizable() -> impl Summary {
	SocialPost {
		username: String::from("horse_ebooks"),
		content: String::from("text"),
		reply: false,
		repost: true
	}
}