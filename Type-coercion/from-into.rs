// From /Into Conversion
//
// 'From ' and 'Into' traits are used for type conversions between different types without
// requiring explicit type casting.
// Part of standard library.
// Can be implemented for custom types.
// Implementing From for a type will give us Into implementing for the given type for free.
//
// eg:
// #[derive(Debug)]
//  struct Number {
//    value: i32,
//    }
//
//    impl From<i32> for Number {
//    fn from(item: i32) -> Number {
//    Number { value: item }
//    }
//    }
//    fn main() {
//    let num = Number::from(30);
//    assert_eq!(30, num.value);
//    let num:Number = 30_i32.into();
//    assert_eq!(30, num.value);
//    }
//
//    From/Into :
//    The From trait allows for a type to create itself from another type, when possible.
//
//    The from and into traits are inherently linked, and this is actually part of its
//    implementation.It means if we write something like this : impl From<T> for U, then we can use
//    let u: U = U::from(T) or let u = T.into().
//
//    The Into trait is simply the reciprocal of the From trait. That is, if you have implemented
//    the From trait for your type, Into trait will be automatically implemented for the same type.
//
//    Using the Inot trait will typically require the type annotations as the compiler is unable to
//    to determine this most of the time.
//
//    For example, we can easily convert &str into String :
Because the standard library has already implemented this for us : impl From<&'_ str> for String .
fn main() {
    let my_str = "hello";

    // three conversions below all depends on the fact: String implements From<&str>:
    let string1 = String::from(my_str);
    let string2 = my_str.to_string();
    // Explicit type annotation is required here
    let string3: String = my_str.into();
}


1.🌟🌟
fn main() {
    // impl From<bool> for i32
    let i1: i32 = false.into();
    let i2: i32 = i32::from(false);
    assert_eq!(i1, i2);
    assert_eq!(i1, 0);

    // FIX the error in two ways
    /* 1. use a similar type which `impl From<char>`, maybe you
    should check the docs mentioned above to find the answer */
    // 2. a keyword from the last chapter
    let i3: u32 = 'a'.into(); // impl From<char> for i32 is not possible as impl From<char> for
    // unsigned integers is only possible as a char cant be negative.

    // FIX the error in two ways
    let s1: String = 'a'.to_string(); // as char does not implement From<String> so we use
    // to_string()
    let s2: String = String::from('a'); // as char does not implement From<String> so we use

    println!("Success!");
}

2.🌟🌟
implementting From for custom types:
// From is now included in `std::prelude`, so there is no need to introduce it into the current scope
// use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Number {
        Number { value: item }
    }
}

// FILL in the blanks
fn main() {
    let num = Number::from(30);
    assert_eq!(num.value, 30);

    let num: Number =30.into();
    assert_eq!(num.value, 30);

    println!("Success!");
}

3. 🌟🌟🌟
when performing error handling ,it is often useful to implement From trait for our own error type.Then we can use ?
to automatically convert the underlying error type to our own error type.
//standard libraries
use std::io; // io::Error
use std::fs; // fs::File
use std::num; // num::ParseIntError
enum CliError {
    IoError(io::Error),
    ParseError(num::ParseIntError),
}

impl From<io::Error> for CliError {

    // IMPLEMENT from method
   fn from(e : io::Error) -> Self {
    CliError::IoError(e)
    }
}


impl From<num::ParseIntError> for CliError {
    // IMPLEMENT from method
    fn from(e: num::ParseIntError ) -> Self {
        CliError::ParseError(value)
    }
}

fn open_and_parse_file(file_name: &str) -> Result<i32, CliError> {
    // ? automatically converts io::Error to CliError
    let contents = fs::read_to_string(&file_name)?;
    // num::ParseIntError -> CliError
    let num: i32 = contents.trim().parse()?;
    Ok(num)
}

fn main() {
    println!("Success!");
}


4.🌟🌟🌟
TryFrom /TryInto:
Similar to From/Into but for fallible conversions.
TryFrom and TryInto are generic traits for converting between types.
Unlike From/Into, the TryFrom and TryInto traits are used for fallible conversions and return a Result instead
of a plain value.


// TryFrom and TryInto are included in `std::prelude`, so there is no need to introduce it into the current scope
// use std::convert::TryInto;

fn main() {
    let n: i16 = 256;

    // Into trait has a method `into`,
    // hence TryInto has a method ?
    let n: u8 = match n.try_into() {
        Ok(n) => n,
        Err(e) => {
            println!("there is an error when converting: {:?}, but we catch it", e.to_string());
            0
        } // if we handele the error then program wont panic and print the error statement.
    };

    assert_eq!(n, 0);

    println!("Success!");
}

5.🌟🌟🌟

#[derive(Debug, PartialEq)]
struct EvenNum(i32);

impl TryFrom<i32> for EvenNum {
    type Error = ();

    // IMPLEMENT `try_from`
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNum(value))
        } else {
            Err(())
        }
    }
}

fn main() {
    assert_eq!(EvenNum::try_from(8), Ok(EvenNum(8)));
    assert_eq!(EvenNum::try_from(5), Err(()));

    // FILL in the blanks
    let result: Result<EvenNum, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNum(8));
    let result: Result<EvenNum, ()> = 5i32.try_into();
    assert_eq!(result, Err(());

    println!("Success!");
}


//convert any type(custom type) to string
//To convert any type to a string , we can simply use ToString trait.
//Rather than doing it directly, we should implement the fmt::Display trait which will
//autmoatically provides ToString and also allows you to print the type using println! macro.
//
//1.🌟🌟
//use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    // IMPLEMENT fmt method
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "The point is ({}, {})", self.x, self.y)
    }
}

fn main() {
    let origin = Point { x: 0, y: 0 };
    // FILL in the blanks
    assert_eq!(origin.to_string(), "The point is (0, 0)");
    assert_eq!(format!("{}",origin), "The point is (0, 0)");
println!("{}", origin);
    println!("Success!");
}

//2.🌟🌟
//Parse a String:
//we can use Parse method to convert a string into a i32 number , this is becuse FromStr is
//implemented for i32 type in standard library: impl FromStr for i32.
//// To use `from_str` method, you need to introduce this trait into the current scope.
use std::str::FromStr;
fn main() {
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();
    let from_str: i32 = i32::from_str("20").unwrap();
    let sum = parsed + turbo_parsed + from_str;
    assert_eq!(sum, 35);

    println!("Success!");
}
