// 'Result' is an enum type that represents the outcome of a operation that may fail.
// Two possible variants are Ok and Err.
// - Ok(T) represents the successful result of an operation
// - Err(E) represents the error result of an operation
// Since Result is an enum, we can use match to handle the result.
//
// Result example :
// fn divide (x: f32, y: f32) -> Result<f32, &'static str> {  // &'static str is a string that
// lives for the entire duration of the program
//   if(y == 0.0) {
//     return Err("Cannot divide by zero");
//     }
//     Ok(x/y)
//     }
//  fn main() {
//  let result = divide(10.0, 0.0);
//  match result {
//  Ok(value) => println!("Result: {}", value),
//  Err(message) => println!("Error: {}", message),
//  }
//
//
//Unwrap :The unwrap() method takes as input a value of type Result<T, E> and returns the value T
//if the result is Ok(T). If the result is Err(E), the method panics and prints the error message
//
// ? operator:
// The ? operator is a shorthand way to propagate errors or unwrap Ok() results.
// Basically same as unwrap, but it will return the error instead of panicking in case of error.
// Replaces an entire match statement.
// can be used in the main() function.
// eg
// fn main() -> Result<(), ParseIntError> {
// let number_str = "10";
// let number = number_str.parse::<i32>(){
// Ok(number) => println!("number: {}", number),
// Err(e) => return Err(e),
// };
// println!("{}", number);
// Ok(())
// }
//
// with ? operator, the above program can be written as:
// fn main() -> Result<(), ParseIntError> {
// let number_str = "10";
// let number = number_str.parse::<i32>()?;
// println!("{}", number);
// Ok(())
// }
//
//Type alias: A type alias is a way to give a new name to an existing type. It is useful when the
//eg
// type Result<T> = Result<T, String>;
// fn main() {
// let result: Result<i32> = Ok(10);
// }
//
// eg2
// type U64 = u64;
// fn main() {
// let x: U64 = 10;
// }
//
// 1.🌟🌟
//
// FILL in the blanks and FIX the errors
use std::num::ParseIntError;

fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError>{
    let n1: Result< i32, ParseIntError> = n1_str.parse::<i32>(); //Ok(10)
    let n2: Result< i32, ParseIntError> = n2_str.parse::<i32>(); //Ok(2)
    Ok(n1.unwrap() * n2.unwrap())
}

fn main() {
    let result: <i32, ParseIntError> = multiply("10", "2");
    assert_eq!(result, Ok(20));

    let result:< i32 , ParseIntError> = multiply("4", "2");
    assert_eq!(result.unwrap(), 8);

    println!("Success!");
}

?
? is almost exactly equivalent to unwrap, but ? returns instead of panic on Err.

2.🌟🌟
use std::num::ParseIntError;

// IMPLEMENT multiply with ?
// DON'T use unwrap here
fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError>__ {
    let n1: Result<i32, ParseIntError> = n1_str.parse::<i32>()?;
    let n2: Result<i32, ParseIntError> = n2_str.parse::<i32>()?;
    Ok(n1 * n2)
}

fn main() {
    assert_eq!(multiply("3", "4").unwrap(), 12);
    println!("Success!");
}

// 3.🌟🌟
//
use std::fs::File; // File is a struct representing a file
use std::io::{self, Read}; // io is a module that provides input and output functionality

fn read_file1() -> Result<String, io::Error> {
    let f: Result<File, io::Error> = File::open("hello.txt");
    let mut f:File = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s: String = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// FILL in the blanks with one code line
// DON'T change any code lines
fn read_file2() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?; //if this fails it returns an io::Error

    Ok(s)
}

fn main() {
    assert_eq!(read_file1().unwrap_err().to_string(), read_file2().unwrap_err().to_string()); //unwrap_err()
    //returns the error value contained in the Result and to_string() converts it to a string
    println!("Success!");
}

4.🌟🌟
map & and_then
map and and_then are two common combinators for Result<T, E> (also for Option<T>).

use std::num::ParseIntError;

// FILL in the blank in two ways: map, and _then
fn add_two(n_str: &str) -> Result<i32, ParseIntError> {
   n_str.parse::<i32>().map(|n| n + 2) // map is used to transform the value inside the Result.it
// takes the unwrapped value and applies the closure to it. It returns a new Result with the
// transformed value.
//map leaves the error value untouched
// second approach or alternate approach is to use and_then
// n_str.parse::<i32>().and_then(|n| Ok(n + 2))
}

fn main() {
    assert_eq!(add_two("4").unwrap(), 6);

    println!("Success!");
}

5.🌟🌟
use std::num::ParseIntError;

// With the return type rewritten, we use pattern matching without `unwrap()`.
// But it's so Verbose...
fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
    match n1_str.parse::<i32>() {
        Ok(n1)  => {
            match n2_str.parse::<i32>() {
                Ok(n2)  => {
                    Ok(n1 * n2)
                },
                Err(e) => Err(e),
            }
        },
        Err(e) => Err(e),
    }
}

// Rewriting `multiply` to make it succinct
// You should use BOTH of  `and_then` and `map` here.
fn multiply1(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
    // IMPLEMENT...
  n1_str.parse::<i32>().and_then(|n1| n2_str.parse::<i32>().map(|n2| n1 * n2))
 //or   n2_str.parse::<i32>().and_then(|n2| n1_str.parse::<i32>().map(|n1| n1 * n2))
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n)  => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    // This still presents a reasonable answer.
    let twenty = multiply1("10", "2");
    print(twenty);

    // The following now provides a much more helpful error message.
    let tt = multiply("t", "2");
    print(tt);

    println!("Success!");
}


// 6.🌟🌟
// Type alias
Using std::result::Result<T, ParseIntError> everywhere is verbose and tedious, we can use alias for this purpose.

use std::num::ParseIntError;

// FILL in the blank
type Res<i32> = Result<i32, ParseIntError>;

// Use the above alias to refer to our specific `Result` type.
fn multiply(first_number_str: &str, second_number_str: &str) -> Res<i32> {
    first_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str.parse::<i32>().map(|second_number| first_number * second_number)
    })
}

// Here, the alias again allows us to save some space.
fn print(result: Res<i32>) {
    match result {
        Ok(n)  => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    print(multiply("10", "2"));
    print(multiply("t", "2"));

    println!("Success!");
}

7.🌟🌟

Using Result in fn main
Typically the main function will look like this:

fn main() {
    println!("Hello World!");
}

However main is also able to have a return type of Result. If an error occurs within the main
function it will return an error code and print a debug representation of the error( Debug trait ).

The following example shows such a scenario:


use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    let number_str: &str = "10";
    let number: i32 = match number_str.parse::<i32>() {  // alternativly we can write
// let number: i32 = number_str.parse::<i32>()?;
        Ok(number)  => number,
        Err(e) => return Err(e),
    };
    println!("{}", number);
    Ok(())
}
