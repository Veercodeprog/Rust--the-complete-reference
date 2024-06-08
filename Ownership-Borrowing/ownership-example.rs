fn main() {
    let s = String::from("hello"); // s comes into scope.
    takes_ownership(s); // s's value moves into the function and so is no longer valid here.i.e s is
                        // invalidated
                        //    println!("{}", s); // this will not work as s has been moved and is no longer valid.
    let x = 5; // x comes into scope.
    makes_copy(x); // x would move into the function but i32 is a copy type and so x is still valid
                   // here.
} // here x goes out of scope but s has already been moved and so nothing happens.

fn takes_ownership(some_string: String) {
    // some_string comes into scope.
    println!("{}", some_string);
} // here some_string goes out of scope and `drop` is called. the backing memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope.
    println!("{}", some_integer);
} // here some_integer goes out of scope. nothing special happens.
  //
