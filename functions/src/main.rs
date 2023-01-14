fn main() {
    // // println!("Hello, world!");
    // another_function(5);
    // print_labled_measurement(5, 'h');
    // let y = {
    //     let x = 3;
    //     x + 1
    // };
    // println!("{y}");
    let x = plus_one(5); //must have data type 
    println!("{x}")
}

// fn another_function(x: i32) {
//     println!("the value of x is {x}");
// }

// fn print_labled_measurement(value: i32, unit_label: char) {
//     println!("The measurement is: {value}{unit_label}")
// }

fn plus_one(x: i32) -> i32 { //declare datatype when the function returns a value
    x + 1
}