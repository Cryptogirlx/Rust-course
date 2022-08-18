fn main() {

    //** PRINTING **//


    // println is console.log

    //print in next line
    println!("Hello, world!");
    println!("my name is {}","Zsofie");
    println!("I have a {} named {}","cat","Gizmo");
    println!("I'm {} yrs old.",30);

    // print on the same line
    print!("This is going to");
    print!("be printed on the same line");

    // named arguments
    println!("");
    println!("{name} is a {animal}",  name = "Gizmo",animal = "cat");
  
   // basic math
   println!("25 + 10 = {}",25+10);



   //** VARIABLES **//

   // need to determine data types
   // all variables are immutable in rust by default, you can define a mutable variable with the "mut" keyword
   
   let x:i32 = 15;
   println!("the value of x = {}",x);

   let mut y:i32 = 15;
   y = 20;
   println!("the value of y = {}",y);

   // you can get the max value of an integer type by:

   println!("the max value of i8 is {}",std::i8::MAX);
      println!("the max value of i16 is {}",std::i16::MAX);
         println!("the max value of i32 is {}",std::i32::MAX);
            println!("the max value of i64 is {}",std::i64::MAX);

 //** FLOATS **/

 // floats can store numbers with decimals
 // you cannot add a float to an integer 
 let float:f32 = 6.66;
 let z:f32 = 5.55;

 println!("{}",float + z);

 //** BOOLEANS **//

 let status:bool = false;
 println!("{}",status);

 let not_equals:bool = 18 !=18;
  println!("{}",not_equals);

 //** CHARACTERS **//

 // characters can represent emojis and other symbols too

 let c1:char = 'a';
 let c2:char = '+';
 let c3:char = '\u{1F608}';

 println!("{},{},{},{}",c3,c1,c2,c3);

 //** VARIABLES **//

//  let(first_number:i32, second_number:f64) = (666,4.20);
//  println!("{},{}"first_number,second_number);
}
