fn main() {
    // if Expression
    let number = 3;
    if number != 0 { //if number is incorrect as it is not boolean
        println!("Number was something other than zero");
    }



    //Handling Multiple Conditions with else if
    let number = 6;
    if number % 4 == 0{
        println!("number is divisible by 4");
    } else if number % 3 ==0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }




    ////Using if in a let Statement
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of the number is {number}");


    // // Using if in a let Statement
    let condition = true;
    let number = if condition { 5 } else { "six "};
    println!("The value of the number is {number}");



    //Repetition with loops   
    loop {
        println!("again!");
    }
    


    //Returning Values from Loops
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");



    //Loop Labels to Disambiguate Between Multiple Loops    
    let mut count = 0;
    println!("count = {count}");
    'counting_up : loop {
        let mut remaining = 10;
        println!("remaining = {remaining}");

        loop {
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");




    //Conditional Loops with while
    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [1 , 2, 3, 4 ,5];

    for element in a {
        println!("the value of element is {element}");
    }
    


    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");


}

