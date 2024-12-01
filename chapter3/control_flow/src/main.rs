fn main() {
    let number = 3;

    if number < 5 {
        println!("number is lesser than 5");
    } else if number < 5 {
        println!("number is greater than 5");
    } else {
        println!("number is equal to 5");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    //let number if condition { 5 } else { "six" }; // compile time error
    println!("number is {number}");

    // Returning Values from Loops
    // In a loop, a semicolon is used when you have a statement inside the loop
    // (such as a function call or an assignment), but you do not need a semicolon
    // after the `break` or `continue` expression if they return a value
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    // Loop Labels to Disambiguate Between Multiple Loops
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
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

    // Loop through a collection with for
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is {element}");
    }

    for number in (1..4).rev() {
        // from 1 to 3
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
