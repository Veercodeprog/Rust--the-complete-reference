//Closures:
//Anonymous function that are able to capture values from the scope in which they are defined.
//Can be defined inline , that means they can be used as arguments to functions or stored in data.
//Dont require type annotation.
//can take ownership of a value by using 'move' keyword before the parameter list.
//
//Fn traits:
//All functions and closures implement one of the traits in the Fn family.
//Fn trait is the trait that defines the signature for the closures/functions.
//Describes types , number of arguments and return type.
//Three traits in the Fn family:
//-FnOnce:
// closeure that can be called once.
// takes ownership of the variables it captures from its enclosing scope.
//
// -FnMut:
// Might mutate the variables it captures.
// can be called multiple times.
//
// -Fn:
// Doesnt take any ownership of the variables it captures.
// doesnt mutate the variables it captures.
// might not even capture naything from it's environment.
//
//
// Closure eg:
fn main() {
 let x=1;
let closure = |val| val + x;
assert_eq!(2,closure(1));
}

this closure captures the value of x from the enclosing scope and modifies it.
in this case , mutable reference of x is taken rather than taking the ownership because it's less restrictive.


Closure:
Closure can capture the enclosing environments .For eg we can capture the x varible :
fn main() {
 let x=1;
let closure = |val| val + x;
assert_eq!(2,closure(1));
}
Unlike functions, both the input and return types of closures can be inferred by the compiler.

Capturing:
Closures can capture variables by borrowing or moving.
But they prefer to capture by borrowing and only go lower when required:
- By reference: &T
- By mutable reference: &mut T
- By value: T

/* Make it work with least amount of changes*/
fn main() {
    let color = String::from("green");

  i//  let print = move || println!("`color`: {}", color);// using move keyword to take ownership of
    // color, i.e. captured value will be moved to the closure.
let print = || println!("`color`: {}", color);
    print();
    print();

    // `color` can be borrowed immutably again, because the closure only holds
    // an immutable reference to `color`.
    let _reborrow = &color;

    println!("{}",color);
}

2.
/* Make it work
- Dont use `_reborrow` and `_count_reborrowed`
- Dont modify `assert_eq`
*/
fn main() {
    let mut count: i32 = 0;

    let mut inc = move || {
        count += 1;  // mutable reference to count is captured by the closure.
        println!("`count`: {}", count);  //program will work fine if we add move keyword here as
    //we can either have one mutable reference or multiple immutable references.
    };

    inc();


    let _reborrow: &i32 = &count;

    inc();

    // The closure no longer needs to borrow `&mut count`. Therefore, it is
    // possible to reborrow without an error
    let _count_reborrowed = &mut count; // mutable reference to count is borrowed again. so error
// will be thrown.

    assert_eq!(count, 0);
}

3.
/* Make it work in two ways, none of them is to remove `take(movable)` away from the code
*/
fn main() {
     let movable: Box<i32>  = Box::new(3);

     let consume = || {
         println!("`movable`: {:?}", movable);
         take(movable);  //ownership of movable is taken by the take function.
     };

     consume();
  //   consume(); // consume() cant be called second time as movable has been moved to take function
    // after first call. of consume.
}

fn take<T>(_v: T) {}


Type inferred
The following four closures has no difference in input and return types.

fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;

4.

fn main() {
    let example_closure = |x: String|-> String { x };

    let s: String = example_closure(String::from("hello"));

    /* Make it work, only change the following line */
    let n = example_closure(5.to_string());:w

}

Fn, FnMut, FnOnce:
When taking a closure as an input parameter, the closure's complete type must be annotated using one of the following traits:

Fn: the closure uses the captured value by reference (&T)
FnMut: the closure uses the captured value by mutable reference (&mut T)
FnOnce: the closure uses the captured value by value (T)

5.
/* Make it work by changing the trait bound, in two ways*/
fn fn_once<F>(func: F)
where
  //  F: FnOnce(usize) -> bool,  // z has to be of usize type.
    F: Fn(usize) -> bool,
{
    println!("{}", func(3)); // ownrship of x is taken by the closure. on first call of func i.e.if
    // we change FnOnce to Fn, it will work fine.
    // fn_once.
    println!("{}", func(4));
}

fn main() {
    let x = vec![1, 2, 3];
    fn_once(|z|{z == x.len()})
}

6.
fn main() {
    let mut s = String::new();

    let update_string = |str| s.push_str(str);

    exec(update_string);

    println!("{:?}",s);
}

/* Fill in the blank */
fn exec<'a, F: FnMut(&'a str) >(mut f: F)  {
    f("hello")
}


Which trait does the compiler prefer to use?

- Fn: when the closure captures by reference.
- FnMut: when the closure captures by mutable reference.
- FnOnce: when the closure captures by value.

On a variable-by-variable basis, the compiler will capture variables in the least restrictive manner possible.

For instance, consider a parameter annotated as FnOnce. This specifies that the closure may capture by &T,
&mut T, or T, but the compiler will ultimately choose based on how the captured variables are used in the closure.
Which trait to use is determined by what the closure does with captured value.

This is because if a move is possible, then any type of borrow should also be possible. Note that the reverse is not true.
If the parameter is annotated as Fn, then capturing variables by &mut T or T are not allowed.

7.
/* Fill in the blank */

// A function which takes a closure as an argument and calls it.
// <F> denotes that F is a "Generic type parameter"
fn apply<F>(f: F) where
    // The closure takes no input and returns nothing.
    F: FnOnce() { // FnOnce trait is used as the closure takes no input and returns nothing and
// also it takes no arguments.

    f();
}

// A function which takes a closure and returns an `i32`.
fn apply_to_3<F>(f: F) -> i32 where
    // The closure takes an `i32` and returns an `i32`.
    F: Fn(i32) -> i32 {

    f(3)
}

fn main() {
    use std::mem;

    let greeting = "hello";
    // A non-copy type.
    // `to_owned` creates owned data from borrowed one
    let mut farewell = "goodbye".to_owned();  // to_owned() creates a String type for &str type.

    // Capture 2 variables: `greeting` by reference and
    // `farewell` by value.
    let diary = || {
        // `greeting` is by reference: requires `Fn`.
        println!("I said {}.", greeting);

        // Mutation forces `farewell` to be captured by
        // mutable reference. Now requires `FnMut`.
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        // Manually calling drop forces `farewell` to
        // be captured by value. Now requires `FnOnce`.
        mem::drop(farewell);
    };

    // Call the function which applies the closure.
    apply(diary);

    // `double` satisfies `apply_to_3`'s trait bound
    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));
}


Move closures may still implement Fn or FnMut, even though they capture variables by move.
This is because the traits implemented by a closure type are determined by what the closure does with
captured values, not how it captures them. The move keyword only specifies the latter.


fn main() {
    let s = String::new();

    let update_string = move || println!("{}",s);

    exec(update_string);
}

fn exec<F: FnOnce()>(f: F)  {
    f()
}

The following code also has no error:


fn main() {
    let s = String::new();

    let update_string = move || println!("{}",s);

    exec(update_string);
}

fn exec<F: Fn()>(f: F)  {  // we can usse Fn trait as well as FnOnce trait as the closure takes no
    // arguments and returns nothing.basically it just prints the value of s in original statemtent
    // .so fn trait can be used here.
    f()
}

8.
/* Fill in the blank */
fn main() {
    let mut s = String::new();

    let update_string = |str| -> String {s.push_str(str); s };  // if we would we only
    // s.push_str(str) then using FnMut trait would have been enough but as we are returning
    // s as well so we need to use FnOnce trait. we need the exec function to take ownership of s
    // in order to return it.

    exec(update_string);
}

fn exec<'a, F: FnOnce (&'a str) -> String >(mut f: F) {
    f("hello");
}

Input functions
Since closure can be used as arguments, you might wonder can we use functions as arguments too? And indeed we can.

9.

/* Implement `call_me` to make it work */
fn call_me<F: Fn()>(f: F) {
    f();
}

fn function() {
    println!("I'm a function!");
}

fn main() {
    let closure = || println!("I'm a closure!");

    call_me(closure);
    call_me(function);
}


Closure as return types
Returning a closure is much harder than you may have thought of.

10.
/* Fill in the blank using two approaches,
 and fix the error */
fn create_fn() -> impl Fn(i32) -> i32 { // static dispatch
    let num = 5; //num must outlive the closure. as it is used in the return statement of the
    //closure.
    //and num scope is limited to create_fn function only.

    // How does the following closure capture the environment variable `num`
    // &T, &mut T, T ?
   move |x| x + num  // as the result is of i32 type so the closure should return i32 type.;w
    //
}
// using dynamic dispatch
fn create_fn1() -> Box<dyn Fn(i32) -> i32> { // dynamic dispatch
    let num = 5; //num must outlive the closure. as it is used in the return statement of the
    //closure.
    //and num scope is limited to create_fn function only.

    // How does the following closure capture the environment variable `num`
    // &T, &mut T, T ?
  Box::new(move |x| x + nu)m  // as the result is of i32 type so the closure should return i32 type.;w
    //
}


fn main() {
    let fn_plain = create_fn();
    // let fn_plain = |x| x+ num;
    //num is not in scope here so it will throw error.but if we use the move keyword in the closure
    //that is being returned then it will work fine as the ownership of num will be taken by the
    //closure.
    fn_plain(1);
}

11
/* Fill in the blank and fix the error*/
fn factory(x:i32) -> Box<dyn Fn(i32) -> i32> {

    let num = 5;

    if x > 1{
       Box::new(move |x| x + num)  // both return type looks same but they are at differennt memory
        // locations . so they are different.so we will use dynamic dispatch here.
    } else {
        Box::new(move |x| x + num)
    }
}

fn main() {
    let f = factory(1);
    let answer = f(1);

    assert_eq!(6, answer);
}
