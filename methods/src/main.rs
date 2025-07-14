#[derive(Debug)] //Lets make the default debug trait availeable for Rectangle
struct Rectangle {
    width:  u32,
    height: u32
}

impl Rectangle {
    fn area(&self ) -> u32 {   //&self is short for self: &Self and is an alias for Rectangle
                               //we use &self because want no ownership
                               //also could be &mut self
                               //simple self is rare, usually we do not want to take ownership, usually a transformation
        self.width * self.height
    }
    
    fn has_width(&self ) -> bool {
        self.width > 0
    }
    
    fn can_hold(&self, rec: &Rectangle ) -> bool {
        (self.width > rec.width) & (self.height > rec.height)
    }
    
    //This is an associated function
    //The not a method!!
    //First parameter is not self
    //Used as a constructor, usually "new()"
    // "Self" is just an alias for "Rectangle"
    fn square(size: u32 ) -> Self {
        Self {
            width:  size,
            height: size
        }
    }
    
    /*
    Why do we add self to the method?
     - This make possible automatic dereferencing!
     - Compiler knows when calling a method on a ref/type - mut/unmut what to do
    */
}

fn main() {
   let rect1: Rectangle = Rectangle {
       width:  30,
       height: 50
   };
   println!(
       "The area of the rectangle is: {}",
       rect1.area()
   );
   
   if rect1.has_width() {
       println!("The rectangle has non-zero width, and it is {}", rect1.width );
   }
   
   let rect2: Rectangle = Rectangle {
       width:  10,
       height: 40
   };
   let rect3: Rectangle = Rectangle {
       width:  60,
       height: 45
   };
   println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2 ) );
   println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3 ) );
   
   let square = Rectangle::square(2);
   println!("The quare is: {:#?}", square );
}