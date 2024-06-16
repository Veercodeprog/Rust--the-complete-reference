//Lifetimes:
//Lifetime is a name for a region of code that some reference must be valid for.
//Another kind of generic ensuring that references are valid as long as the data they point to is
//valid.
//Every reference has a lifetime, which is the scope for which that reference is valid.
//Most of the time implicit and inferred , don't need to be annotated.
//Sometimes lifetime annotations are required, if teh compiler can't figure out the lifetimes or
//annotate it .
//Lifetime annotation is the concept which most programming langauges don't have.
//
//Dangling references:
//Main aim of lifetime is to avoid danglinf references.
//
//Borrow checker:
//Borrow checker compares scopes to determine whether all borrows are valid.
//Track lifetimes of references and ensures they dont violate the ownership rules.
//Rules ensure that a value is not accessed after it has been moved or freed from memory.
//a reference to a value must not outlive the value it points to.
//
//1.
///* Annotate the lifetime of `i` and `borrow2` */

// Lifetimes are annotated below with lines denoting the creation
// and destruction of each variable.
// `i` has the longest lifetime because its scope entirely encloses
// both `borrow1` and `borrow2`. The duration of `borrow1` compared
// to `borrow2` is irrelevant since they are disjoint.
fn main() {
    let i = 3;
    {
        let borrow1 = &i; // `borrow1` lifetime starts. ──┐
                          //                                                │
        println!("borrow1: {}", borrow1); //              │
    } // `borrow1 ends. ──────────────────────────────────┘
    {
        let borrow2 = &i;

        println!("borrow2: {}", borrow2);
    }
}

2.

{
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("r: {}", r); //   |       |
                          // --+       |
}                         // ----------+


Lifetime annotating
The borrow checker uses explicit lifetime annotations to determine how long a reference should be valid.

But for us users, in most cases, there is no need to annotate the lifetime, because there are several elision rules,
before learning these rules, we need to know how to annotate lifetime manually.

Function
Ignoring elision rules, lifetimes in function signatures have a few constraints:

Any reference must have an annotated lifetime
Any reference being returned must have the same lifetime as one of the inputs or be static

example:
// One input reference with lifetime `'a` which must live
// at least as long as the function.
fn print_one<'a>(x: &'a i32) {  // 'a is a lifetime i.e. lifetime of x must outlive the function as
        // it is passed as reference to the function.
    println!("`print_one`: x is {}", x);
}

// Mutable references are possible with lifetimes as well.
fn add_one<'a>(x: &'a mut i32) {
    *x += 1;
}

// Multiple elements with different lifetimes. In this case, it
// would be fine for both to have the same lifetime `'a`, but
// in more complex cases, different lifetimes may be required.
fn print_multi<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("`print_multi`: x is {}, y is {}", x, y);
}

// Returning references that have been passed in is acceptable.
// However, the correct lifetime must be returned.
fn pass_x<'a, 'b>(x: &'a i32, y: &'b i32) -> &'a i32 { x }

fn main() {
    let x:i32 = 7;
    let y:i32 = 9;

    print_one(&x);
    print_multi(&x, &y);

    let z = pass_x(&x, &y);
    print_one(z);

    let mut t = 3;
    add_one(&mut t);
    print_one(&t);
}

3
/* Make it work by adding proper lifetime annotation */
fn longest<'a>(x: &'a str, y:'a &str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
let x: &str = "long";
let y: &str = "longer";
    println!("The longest string is {}", longest(x, y));
}

4.
// `'a` must live longer than the function.
// Here, `&String::from("foo")` would create a `String`, followed by a
// reference. Then the data is dropped upon exiting the scope, leaving
// a reference to invalid data to be returned.

/* Fix the error in three ways  */
fn invalid_output<>() -> String {
   // &String::from("foo")  // returning a reference to a string that is dropped after the function
    // end , to fix this return a string instead of a reference or return a string literal "foo".in
    // this case return type would be &'static str instead of String because string literal live
    // long as the program runs.
String::from("foo")
}

fn main() {
    let x: String = invalid_output(); // Error: borrowed value does not live long enough i.e. x
    // reference to a string that is dropped after the function ends
    println!("x is {}", x);
}


//alternative solution
fn invalid_output<'a>(s: &'a str) -> &'a str {
    s
}
fn main() {
  let s: String = String::from("foo");  //nlifetime is based on the concept of passing a refernece
    //to a string in a function and ensuring that s must lives even after the function ends . so
    //that when the reference is returned it is valid.otherwise if s is dropped after the function
    //then the reference would be invalid.
    let x: &str = invalid_output(&s);
    println!("x is {}", x);
}

5.
// `print_refs` takes two references to `i32` which have different
// lifetimes `'a` and `'b`. These two lifetimes must both be at
// least as long as the function `print_refs`.
fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y);
}

/* Make it work */
// A function which takes no arguments, but has a lifetime parameter `'a`.
fn failed_borrow<>() {
    let _x = 12;

    // ERROR: `_x` does not live long enough
    let y: &i32 = &_x;// we have to remove the lifetime parameter from the function signature
    // as y wont outlive the function.
    // Attempting to use the lifetime `'a` as an explicit type annotation
    // inside the function will fail because the lifetime of `&_x` is shorter
    // than `'a` . A short lifetime cannot be coerced into a longer one.
}

fn main() {
    let (four, nine) = (4, 9);

    // Borrows (`&`) of both variables are passed into the function.
    print_refs(&four, &nine);
    // Any input which is borrowed must outlive the borrower.
    // input : &four, &nine, borrower : print_refs i.e. lifetime of four and nine must be longer
    // and outlive the function print_refs.
    // In other words, the lifetime of `four` and `nine` must
    // be longer than that of `print_refs`.

    failed_borrow();
    // `failed_borrow` contains no references to force `'a` to be
    // longer than the lifetime of the function, but `'a` is longer.
    // Because the lifetime is never constrained, it defaults to `'static`.
}

6.
/* Make it work by adding proper lifetime annotation */

// A type `Borrowed` which houses a reference to an
// `i32`. The reference to `i32` must outlive `Borrowed`.
#[derive(Debug)]
struct Borrowed<'a>(&'a i32);

// Similarly, both references here must outlive this structure.
#[derive(Debug)]
struct NamedBorrowed<'a> {
    x: &'a i32,
    y: &'a i32,
}

// An enum which is either an `i32` or a reference to one.
#[derive(Debug)]
enum Either<'a> {
    Num(i32),
    Ref(&'a i32),
}

fn main() {
    let x:i32  = 18;
    let y:i32 = 15;

    let single: Borrowed = Borrowed(&x);
    let double: NamedBorrowed = NamedBorrowed { x: &x, y: &y };
    let reference: Either = Either::Ref(&x);
    let number: Either    = Either::Num(y);

    println!("x is borrowed in {:?}", single);
    println!("x and y are borrowed in {:?}", double);
    println!("x is borrowed in {:?}", reference);
    println!("y is *not* borrowed in {:?}", number);
}

7.
/* Make it work */

#[derive(Debug)]
struct NoCopyType {}

#[derive(Debug)]
struct Example<'a, 'b> {
    a: &'a u32,
    b: &'b NoCopyType
}

fn main()
{
  /* 'a tied to fn-main stackframe */
  let var_a = 35; //lifetime a' of var_a is longer than lifetime b' of var_b
  let example: Example;

  {
    /* Lifetime 'b tied to new stackframe/scope */
    let var_b: NoCopyType = NoCopyType {};

    /* fixme */
    example: Example = Example { a: &var_a, b: &var_b };
    println!("(Success!) {:?}", example);
    } // problem is var_b is dropped after the scope ends so the reference to var_b is invalid. and
    // we are printin the example which has a reference to var_b.

 // println!("(Success!) {:?}", example);//solution is to move the example inside the scope so that
    // it is dropped after the scope ends.
}


8.


#[derive(Debug)]
struct NoCopyType {}

#[derive(Debug)]
#[allow(dead_code)]
struct Example<'a, 'b> {
    a: &'a u32,
    b: &'b NoCopyType
}

/* Fix function signature */
fn fix_me<'a>(foo: &'a Example) -> &'a NoCopyType
{ foo.b }

fn main()
{
    let no_copy: NoCopyType = NoCopyType {};
    let example: Example  = Example { a: &1, b: &no_copy };
    fix_me(&example);
    println!("Success!")
}

Method
Methods are annotated similarly to functions.

Example

struct Owner(i32);

impl Owner {
    // Annotate lifetimes as in a standalone function.
    fn add_one<'a>(&'a mut self) { self.0 += 1; }
    fn print<'a>(&'a self) {
        println!("`print`: {}", self.0);
    }
}

fn main() {
    let mut owner = Owner(18);

    owner.add_one();
    owner.print();
}

9.
/* Make it work by adding proper lifetime annotations */
struct ImportantExcerpt {
    part: &'static str,
}

impl ImportantExcerpt {
    fn level( self) -> i32 {
        3
    }
}

fn main() {
let importnt: ImportantExcerpt = ImportantExcerpt { part: "A very important part" }'

}
