fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // Condition must be a `bool`. This does not compile:
    // if number {
    //     println!("number was three");
    // }
    // This does, though:
    if number != 0 {
        println!("number was something other than zero");
    }

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 }; // `if` is an expression
    println!("The value of number is: {number}");
    // This does not compile, as both branches must return the same type:
    // let number = if condition { 5 } else { "six" };

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // result = 20
        }
    };
    println!("The result is {result}");

    let mut count = 0;
    'counting_up: loop {
        // Loop labels help disambiguate between multiple loops
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break; // exit the inner loop only
            }
            if count == 2 {
                break 'counting_up; // exit the outer loop
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    let mut number = 3;
    while number != 0 {
        println!("{number}!");

        number -= 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    // Iterating the hard way:
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }
    // Much easier and safer:
    for element in a {
        println!("the value is: {element}");
    }
    // `while` loops can often be replaced with safer `for` loops:
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
