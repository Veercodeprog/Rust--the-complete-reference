fn main() {
    let (x, y);

    (x, ..) = (3, 2); // destructure the tuple
    [.., y] = [1, 2, 3]; // destructure the array
                         //
    assert_eq!([x, y], [3, 3]); // test the values
                                //
    println!("Success!");
}
