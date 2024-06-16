// Debug & Display:
// Types which want to be printable Debug and Display traits must be implemented.
// Automatic implementations are only provided for types such as in the std library.
// Debug trait can be implemented simply by using the derivable trait Debug.
// Display trait must be manually implemented.
//
// println! and format!
Printing is handled by a series of [macros][macros] defined in [std::fmt][fmt] Some of which include:

format!: write formatted text to [String][string]
print!: same as format! but the text is printed to the console (io::stdout).
println!: same as print! but a newline is appended.
eprint!: same as format! but the text is printed to the standard error (io::stderr).
eprintln!: same as eprint!but a newline is appended.

//format:
//1.
//
//
fn main() {
    let s1 = "hello";
    /* Fill in the blank */
    let s = format!("{}, world!", s1);
    assert_eq!(s, "hello, world!");
}

Debug and Display:
All types which want to be printable must implement the std::fmt formatting trait:
std::fmt::Debug or std::fmt::Display.

Automatic implementations are only provided for types such as in the std library.
All others must be manually implemented.


Debug :
The implementation of Debug is very straightforward:All types can derive the std::fmt::Debug implementation.
This is not true for Display which must be manually implemented.

{:?} must be used to print out the type which has implemented the Debug trait.

1.🌟

/* Fill in the blanks and Fix the errors */
#[derive(Debug)]
struct Structure(i32);

fn main() {
    // Types in std and Rust have implemented the fmt::Debug trait
    println!("{}  months in a year.", 12);

    println!("Now {:?} will print!", Structure(3));
}

2.🌟🌟
So fmt::Debug definitely makes one type printable, but sacrifices some elegance. Maybe we can get more elegant by
replacing {:?} with something else( but not {} !)

#[derive(Debug)]
struct Person {
    name: String,
    age: u8
}

fn main() {
    let person = Person { name:  "Sunface".to_string(), age: 18 };

    /* Make it output:
    Person {
        name: "Sunface",
        age: 18,
    }
    */
    println!("{:#?}", person);  // added # to make it more elegant
}

3.🌟🌟🌟

use std::fmt;
#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

    impl sfmt::Debug for Deep {
      fn fmt (&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.0)
      }


fn main() {
    // The problem with `derive` is there is no control over how
    // the results look. What if I want this to just show a `7`?

    /* Make it print: Now 7 will print! */
    println!("Now {:?} will print!", Deep(Structure(7)));
}


Display
Yeah, Debug is simple and easy to use. But sometimes we want to customize the output appearance of our type. This is where Display really shines.

Unlike Debug, there is no way to derive the implementation of the Display trait, we have to manually implement it.

Another thing to note: the placeholder for Display is {} not {:?}.

4.🌟🌟🌟

/* Make it work*/
use std::fmt;

struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    /* Implement.. */
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Display: {} + {}i", self.x, self.y)
    }
}

impl fmt::Debug for Point2D {
    /* Implement.. */
     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Debug: Complex {{ real:{} + imag: {} }}", self.x, self.y)
    }
}

fn main() {
    let point = Point2D { x: 3.3, y: 7.2 };
    assert_eq!(format!("{}",point), "Display: 3.3 + 7.2i");
    assert_eq!(format!("{:?}",point), "Debug: Complex { real: 3.3, imag: 7.2 }");

    println!("Success!");
}


? operator
Implementing fmt::Display for a structure whose elements must be handled separately is tricky.
The problem is each write! generates a fmt::Result which must be handled in the same place.

Fortunately, Rust provides the ? operator to help us eliminate some unnecessary codes for dealing with fmt::Result.:w


/* Make it work */
use std::fmt;

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Extract the value using tuple indexing,
        // and create a reference to `vec`.
        let vec: &Vec<i32> = &self.0;

        write!(f, "[")?;

        // Iterate over `v` in `vec` while enumerating the iteration
        // count in `count`.
        for (count, v) in vec.iter().enumerate() {
            // For every element except the first, add a comma.
            // Use the ? operator to return on errors.
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}: {}",count, v)?;
        }

        // Close the opened bracket and return a fmt::Result value.
        write!(f, "]")
    }
}

fn main() {
    let v = List(vec![1, 2, 3]);
    assert_eq!(format!("{}",v), "[0: 1, 1: 2, 2: 3]");
    println!("Success!");
}

