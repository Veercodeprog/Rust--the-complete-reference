// Char: Single character of size 4 bytes
// Bool: true or false of size 1 byte
// Unit type: () of size 0 bytes, used to return nothing from a function or expressions

use std::mem::size_of_val;

fn main() {
    let c1: char = 'a'; // it is char not string .string is markd by double quotes.
    assert_eq!(size_of_val(&c1), 4);
    println!("size of c1: {}", size_of_val(&c1));
    let c2: char = 'b';
    assert_eq!(size_of_val(&c2), 4);
    println!("size of c2: {}", size_of_val(&c2));

    //unit type: ()
    // it is a type that has only one value: ()i.e. an empty tuple or does not have any value.
    // it is used when no other value is needed.
    let _v: () = ();
    let v: (i32, i32) = (2, 3);
    assert_eq!(_v, implicitly_ret_unit());
    println!("success");

    //size of unit type
    let unit: () = ();
    assert!(size_of_val(&unit) == 0);
    println!("size of unit: {}", size_of_val(&unit));
}
//if a function does not return a value, it is implicitly returning a unit type.
fn implicitly_ret_unit() {
    println!("i will return a ()");
}

fn explicitly_ret_unit() -> () {
    println!("i will return a ()");
}
