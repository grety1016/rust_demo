#[allow(dead_code)]
#[allow(unused_imports)]

use hello_world;
fn main() {
   //hello_world::hello(); 

   let hello = String::from("he llo");

   let hello_bytes = hello.as_bytes();

   for (i, &item) in hello_bytes.iter().enumerate(){
      if item == b' '{
         println!("{0},{1}",i,item);
      }
      
   }
}
