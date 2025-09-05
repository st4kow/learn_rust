use trait_object::Draw;
use trait_object::{Button, Screen};


/* User implementation for Drawable type */
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}
/* ------------------------------------ */

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox { //This is a trait object
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button { //This is a trait object
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}