use std::convert::TryInto;
fn main()
{
    let _a:i32 = 19;

    let million:i64 = 1_000_000;
    //make it to the power of the two
    println!("Million to the power of two is {}",million.pow(2));

    //declare the arrays
    let forty_twos:[f32;3] = [
        4.0_f32,
        4.0,
        4.0f32
    ];

    //pricision hmm
    println!("Precision of 0th element {:03} ",forty_twos[0]);

    let three = 0b11;
  let thirty = 0o36;
  let three_hundred = 0x12C;
 
  println!("base 10: {} {} {}", three, thirty, three_hundred);
  println!("base 2: {:b} {:b} {:b}", three, thirty, three_hundred);
  println!("base 8: {:o} {:o} {:o}", three, thirty, three_hundred);
  println!("base 16: {:x} {:x} {:x}", three, thirty, three_hundred);

 
  
  let a: i32 = 10;
  let b: u16 = 100;
 
  //unwrap gives the success value from the result
  let b_ =  b.try_into().unwrap();
 
 if a < b_ {
 println!("Ten is less than one hundred.");
 }
 
}