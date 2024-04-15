mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;
// This also works but is considered less idiomatic for functions.
use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    // This also works but is considered less idiomatic for functions.
    add_to_waitlist();
}

mod customer {
    use crate::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist()
    }
}

// When bringing in structs, enums, and other items with `use`, using the full
// path is considered more idiomatic.
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

// The exception to this idiom is if we’re bringing two items with the same
// name into scope with use statements, because Rust doesn’t allow that.

use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    todo!();
}

fn function2() -> io::Result<()> {
    todo!();
}

// If instead we specified use std::fmt::Result and use std::io::Result, we’d
// have two Result types in the same scope and Rust wouldn’t know which one we
// meant when we used Result.

// There’s another solution to the problem of bringing two types of the same
// name into the same scope with use: after the path, we can specify as and a
// new local name, or alias, for the type.

use std::fmt::Result;
use std::io::Result as IoResult;

fn function1_with_as() -> Result {
    todo!()
}

fn function2_with_as() -> IoResult<()> {
    todo!();
}
