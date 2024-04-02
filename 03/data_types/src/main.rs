fn main() {
    // Scalar Types

    // Integer Types

    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A'; // u8 only

    // Floating-Point Types

    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    // Numeric Operations

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

    // The Boolean Type

    let t = true;

    let f: bool = false; // with explicit type annotation

    // The Character Type

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // Compound Types

    // The Tuple Type

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    let unit = ();

    // The Array Type

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let b = [3; 5]; // = [3, 3, 3, 3, 3]

    let first = a[0];
    let second = a[1];

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
}
