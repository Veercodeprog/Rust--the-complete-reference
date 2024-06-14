// statements ends with a semicolon.
//
// statements are expressions that performs some action but do not produce or return a value.
// expressions evaluate to a value.
// always remember in rust the return statement does not have a semicolon.
fn main() {
    let x: u32 = 5u32;
    let y: u32 = {
        // all the code block is an expression under {} as it produces a value.
        let x_squared = x * x;
        let x_cube = x_squared * x;
        x_cube + x_squared + x // This is an expression that returns a value as it does not end with a
                               // semicolon and is assigned to y.whenever we have a line with no semicolon, it gets returned
    }; // this is a statement as it ends with a semicolon.
       //
    let z: u32 = {
        // the semicolon suppresses the value of the expression and returns nothing. so '()' is returned
        // and assigned to z.
        2 * x // this is an expression  that returns a value as it does not end with a semicolon.
    }; // this is a statement as it ends with a semicolon.
    println!("x is {}", x);
    println!("y is {}", y);
    println!("z is {}", z);

    let v = {
        // this will return () as we are only incrementing x and not returning it.
        let mut x = 5;
        x += 1
    };
    assert_eq!(v, ());
}
