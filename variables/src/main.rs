fn main() {

    //MUTABILITY
    let mut x = 5; //Mutability is important in Rust if binded to 5 cannot 6 if not the datatype is not mut
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");

    //CONSTANT
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    //SHADOWING
    let x = 5;
    let x = x +1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is {x}");
    }
    println!("The value of x is: {x}"); 
}
