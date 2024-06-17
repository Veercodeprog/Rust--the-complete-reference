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

//&'static and T: 'static:
//
//// A reference with 'static lifetime:
let s: &'static str = "hello world";

// 'static as part of a trait bound:
fn generic<T>(x: T) where T: 'static {}

&'static:
As a reference , &'static indicates the data pointed by the reference lives for the entire duration of the program.
But it can be coerced to a shorter lifetime.

1.🌟🌟 There are several ways to make a variable with 'static lifetime, two of them are stored in the read-only
memory of the binary.


/* Fill in the blank in two ways */
fn main() {
    let v: &'static str  = "hello"; //string literal, we dont need to annotate 'static here as it
    //is inferred by compiler.
    need_static(v);

    println!("Success!")
}

fn need_static(r : &'static str) {
    assert_eq!(r, "hello");
}

2.🌟🌟🌟🌟 Another way to make 'static lifetime is using Box::leak
// do it yourself
// it is a unsafe function that leaks a Box<T> and returns a &'static T
#[derive(Debug)]
struct Config {
    a: String,
    b: String,
}
static mut config: Option<&mut Config> = None;

/* Make it work without changing the function signatures of `init`*/
fn init() -> Option<&'static mut Config> {
    Some(&mut Config {
        a: "A".to_string(),
        b: "B".to_string(),
    })
}


fn main() {
    unsafe {
        config = init();

        println!("{:?}",config)
    }
}

3.
🌟 &'static only indicates that the data can live forever, not the reference. The latter one will be constrained by its scope.

fn main() {
    {
        // Make a `string` literal and print it:
        let static_string: &'static str = "I'm in read-only memory";
        println!("static_string: {}", static_string);

        // When `static_string` goes out of scope, the reference
        // can no longer be used, but the data remains in the binary.
    }

    println!("static_string reference remains alive: {}", static_string);
}

4.&'static can be coerched to a shorter lifetime.
eg
// Make a constant with `'static` lifetime.
static NUM: i32 = 18;
const NUM: i32 = 18;
// both const and static live for the entire duration of the program.
// difference is static will alway remain at the same memory location ,but const get inlines
// meaning compiler will copy into the function ,memory locaion could be change every time we run
// the program.

// Returns a reference to `NUM` where its `'static`
// lifetime is coerced to that of the input argument.
fn coerce_static<'a>(_: &'a i32) -> &'a i32 { // we can coerce the 'static lifetime to a shorter
    // lifetime 'a  ('static > 'a)
    &NUM
}

fn main() {
    {
        // Make an integer to use for `coerce_static`:
        let lifetime_num: i32 = 9;

        // Coerce `NUM` to lifetime of `lifetime_num`:
        let coerced_static: &i32 = coerce_static(&lifetime_num);

        println!("coerced_static: {}", coerced_static);
    }

    println!("NUM: {} stays accessible!", NUM);
}

T: 'static
As a trait bound, it means the type does not contain any non-static references. Eg. the receiver
can hold on to the type for as long as they want and it will never become invalid until they drop it.

It's important to understand this means that any owned data always passes a 'static lifetime bound,
but a reference to that owned data generally does not.

/* Make it work */
use std::fmt::Debug;

fn print_it<T: Debug + 'static>( input: T) { // T: 'static means T does not contain any non-static
    // references
    println!( "'static value passed in is: {:?}", input );
}

fn print_it1( input: impl Debug + 'static ) {
    println!( "'static value passed in is: {:?}", input );
}


fn print_it2<T: Debug + 'static>( input: &T) {
    println!( "'static value passed in is: {:?}", input );
}

fn main() {
    // i is owned and contains no references, thus it's 'static:
    let i: i32 = 5;
    print_it(i);

    // oops, &i only has the lifetime defined by the scope of
    // main(), so it's not 'static:, so this will not work
    // to fix this we can wither make the i cont i.e. const i:i32 = 5; or
    // we can use the static keyword to make it 'static i.e. static i:i32 = 5;
    print_it(&i);

    print_it1(&i);

    // but this one WORKS !
    print_it2(&i);
}
