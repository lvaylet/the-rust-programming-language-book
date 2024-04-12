fn main() {
    #[derive(Debug)]
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    println!("home is {:?}", home);
    println!("loopback is {:?}", loopback);

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            match self {
                Message::Quit => println!("Quitting..."),
                Message::Move { x, y } => println!("Moving to coordinates x={}, y={}...", x, y),
                Message::Write(s) => println!("Writing '{}' to screen ...", s),
                Message::ChangeColor(r, g, b) => {
                    println!("Changing color to R{}, G{}, B{}...", r, g, b)
                }
            }
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();

    let m2 = Message::Move { x: 5, y: -1 };
    m2.call();

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // This code does not compile because i8 and Option<i8> are different types.
    // let sum = x + y;
}
