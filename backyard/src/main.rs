use crate::garden::vegetables::Asparagus;
pub mod garden; //This includes src/garden.rs, makes it public, mod: module

fn main() {
    let plant = Asparagus {};
    println!("I am growing plant {:?}",plant);
}
