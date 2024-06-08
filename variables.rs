// Fix the error below with least amount of modification to the code
// variables are assigned using let keyword
// scope of the variable is from the point of declaration to the end of the block
// shadowing allows a variable to be redefined in the same scope with the same name
// variable can be used if it is initialized
fn main() {
    // let x: i32; // Uninitialized but used, ERROR

    // let (x,y); // decalring multiple variables at once
    let x: i32 = 5; // Initialized and used, No Error
    let y: i32; // Uninitialized but also unused, only a Warning
                // to avoid the warning, we can add _ i.e. _y
                //
                // we can use mut to make the variable mutable
    let mut z: i32 = 10;
    assert_eq!(x, 5);
    println!("Success!");
}
