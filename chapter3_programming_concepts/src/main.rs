use std::io;
fn main() {
    let  num = 3;
    let  num = 3;
    let eightBitNum:i32= 255;
    
    //bools
    let t = true;

    let f: bool = false; // with explicit type annotation

    //floats
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    //numeric operations
        // addition
        let sum = 5 + 10;

        // subtraction
        let difference = 95.5 - 4.3;
    
        // multiplication
        let product = 4 * 30;
    
        // division
        let quotient = 56.7 / 32.2;
        let truncated = -5 / 3; // Results in -1
    
        // remainder
        let remainder = 43 % 5;

        //rust have tuples too like python
        //a tuples is of fixed length and can have different types
        let tup:(i32,String,bool,f32) = (3220,String::from("Nirlin"),true,98.989);
        
        //access the tuple elements
        let (integer, name, truth, point) = tup;
        println!("Integer: {integer}, Name: {name}, truth: {truth}, point: {point}");

        //Arrays
        //Arrays in rust have fixed length as opposed to the other languages where it can have the dynamic length

        println!("Here starts the program that takes the arary index from the use and prints it! Try accessing an out of bound index");
        let a = [1, 2, 3, 4, 5];

        println!("Please enter an array index.");
    
        let mut index = String::new();
    
        io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line");
    
        let index: usize = index
            .trim()
            .parse()
            .expect("Index entered was not a number");
    
        let element = a[index];
    
        println!("The value of the element at index {index} is: {element}");
    
}
