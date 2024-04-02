fn main() {
    // Variables
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // Constants MUST be annotated.
    println!("There are {THREE_HOURS_IN_SECONDS} seconds in 3 hours.");

    // Shadowing
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");

    let spaces = "   ";
    let spaces = spaces.len();
    // This does not compile:
    // let mut spaces = "   ";
    // spaces = spaces.len();
}
