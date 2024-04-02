fn main() {
    println!("Hello, world!");

    another_function(5);

    // Calling a function is an expression.
    print_labeled_measurement(5, 'h');

    let y = 6; // = statement = perform some action and do not return a value
    // let x = (let y = 6); // does not work, as a statement cannot be assigned

    let y = 5 + 1; // 5 + 1 is an expression, like 6 before

    // A new scope blockc reated with curly brackets is an expression:
    let y = {
        let x = 3;
        x + 1 // expression, with no semicolon at the end!
        // x + 1; // statement, with a semicolon, does not return any value
    };

    println!("The value of y is: {y}"); // y = 4

    let x = five();

    println!("The value of x is: {x}");

    let x = plus_one(5); // or plus_one(five())

    println!("The value of x is: {x}");
}

// Function definitions are statements too.
fn another_function(x: i32) {
    // Calling a macro is an expression.
    println!("The value of x is: {x}")
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}")
}

fn five() -> i32 {
    5 // no semicolon => expression whose value we want to return
}

fn plus_one(x: i32) -> i32 {
    x + 1
    // x + 1; // this is an error, as () != i32
}
