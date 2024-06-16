// elision
// Some lifetime patterns are so common that borrow checker will allow you to omit them to save typing and improve readability.
// This is called lifetime elision.
//
// There are three rules of lifetime elision:-
// The compiler uses three rules to figure out what lifetimes references have when there aren't
// explicit annotations.
// 1.Compiler assigns a lifetime parameter for each reference in the function's arguments.
// 2.If there is exactly one input lifetime parameter, that lifetime is assigned to all output
//   lifetime parameters.
// 3.If there are multiple input lifetime parameters, but one of them is &self or &mut self, then
//  the lifetime of self is assigned to all output lifetime parameters.
//
// 1.
// /* Remove all the lifetimes that can be elided */
fn input<'a>(x: &'a i32) {
    //there is only one input lifetime parameter
    println!("`annotated_input`: {}", x);
}

fn pass(x: &i32) -> &i32 {
    //there is only one input lifetime parameter, so the output
    //lifetime parameter will be same as input lifetime parameter
    x
}

fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    x
}

struct Owner(i32);

impl Owner {
    // Annotate lifetimes as in a standalone function.
    fn add_one(&mut self) {
        //if there is a self reference, then the lifetime of self is assigned
        //to all output lifetime parameters and we dont need to annotate it.
        self.0 += 1;
    }
    fn print(&self) {
        println!("`print`: {}", self.0);
    }
}

struct Person<'a> {
    age: u8,
    name: &'a str,
}

enum Either<'a> {
    Num(i32),
    Ref(&'a i32),
}

fn main() {}


Static Lifetime:
refers to a lifetime that lasts for the entire duration of the program's execution.
any reference or borrowed value with 'static lifetime can live for the entire duration of the program.
can be coerced to a shorter lifetime if needed.

string literals have 'static lifetime because they are stored in the binary of the program or they are hardcoded in the program.
