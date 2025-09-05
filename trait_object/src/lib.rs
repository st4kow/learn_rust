pub trait Draw {
    fn draw(&self); //todo;
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>> /* Trait object are always in & or other smart pointers */
}
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw() /* component is a trait object */
        }
    }
}


pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}
impl Draw for Button { /* Implementing Draw trait */
    fn draw(&self) {
        // code to actually draw a button
    }
}

/*
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
*/
