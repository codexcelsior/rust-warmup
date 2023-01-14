use std::io;

fn main() {
    // ** MUTABILITY AND VARIABLES **
    //Mutability
    let mut x = 5; //Mutability is important in Rust if binded to 5 cannot 6 if not the datatype is not mut
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");

    //Constnat
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    //Shadowing
    let x = 5;
    let x = x +1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is {x}");
    }
    println!("The value of x is: {x}"); 

    // let mut spaces = "    ";
    // spaces = spaces.len();

    //** DATA TYPES **
    let guess: u32 = "42".parse().expect("Not a number");

    //signed variable store numbers from -(2^n-1) to (2^n-1) -1 
    //8bit 16bit 32bit 64bit 128bit and arch depends on your system 64 or 32 bit


    //Floating-Point Types
    let x = 2.0; //f64

    let y: f32 = 3.0;//f32
    
    //Numeric Operations
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncated = -5/3;

    let remainder = 43 % 5;
    
    //The Boolean Type
    let t = true;
    let f: bool = false;

    //The Character Type
    let c = 'z';
    let z: char = 'Z';
    let heart_eyed_cat = '😻';

    //The Tuple Type
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let tup = (500, 6.5, 1);
    let (x, y ,z) = tup;

    println!("the value of y is: {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0; //access the first number of the tuple
    let six_point_four = x.1;
    let one = x.2;
    
    println!("{five_hundred}, {six_point_four}, {one}");

    //The Array Type
    let a = [1, 2, 3 ,4 ,5];

    let month = ["January", "February", "March", "April", "May", "June", "July",
    "August", "September", "October", "November", "December"]; //A bit special - String cannot be declared in type of array

    //Example for types of Array: 
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5]; //assign  3 to all values and 5 values in array

    
    //Accessing Array Elements
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    
    //Invalid Array Element Access
    let a = [1, 2, 3, 4, 5];
    println!("Please enter an arry index");
    let mut index = String::new();

    io::stdin().read_line(&mut index).expect("Failed to read line");

    let index: usize = index.trim().parse().
        expect("Index entered is not a number");
    
    let element = a[index];

    println!("The value of the element at index {index} is: {element}")

}
