#[derive(Debug)] //Lets make the default debug trait availeable for Rectangle
struct Rectange {
    width:  u32,
    height: u32
}

fn main() {
   let width1:  u32 = 30;
   let height1: u32 = 50;
   println!(
       "The area of the rectange is: {}",
       area_u32(width1, height1)
   );
   
   //Tuple solution
   println!("Tuple solution:");
   let rect1: (u32, u32) = (30, 50);
   println!(
       "The area of the rectange is: {}",
       area_tuple(rect1)
   );
   
   //Struct solution
   println!("Struct solution:");
   let scale: u32 = 2;
   let rect1: Rectange = Rectange {
       width:  dbg!(30 * scale), //Added debug functionality
       height: 50
   };
   println!(
       "The area of the rectange is: {}",
       area_struct(&rect1)
   );
   
   println!("Printng structs:");
   //println!("rect1 is: {}",rect1 ); //Does not work, because struct do no impelment Display trait
   println!("rect1 is: {:?}",rect1 ); //:? means Debug. This works, because we impelmented Debug trait
   println!("rect1 is: {:#?}",rect1 ); // Same but pretty print
   println!("rect1 is: {rect1:#?}"); //Same but not param
   
   println!("Using Debug output - stderr");
   dbg!(&rect1); //dbg! takes ownership, so give only the reference, also return the value given
   
}

fn area_u32(width: u32, height: u32 ) -> u32 {
    width * height
}

fn area_tuple(dim: (u32, u32 ) ) -> u32 {
    dim.0 * dim.1
}

fn area_struct(rec: &Rectange ) -> u32 {
    rec.width * rec.height
}
