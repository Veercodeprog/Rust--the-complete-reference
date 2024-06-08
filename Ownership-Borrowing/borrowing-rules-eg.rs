fn main() {
    let mut s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM as we can either have multiple immutable references or one mutable
                     // reference but not both.
    println!("{}, {}, and {}", r1, r2, r3);
}

fn main_eg() {
    let mut s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
                 //
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point but if we try to use them after this point, the
    // compiler will not allow us to do so. as the compiler will not allow us to have a mutable
    // reference to the value s as we have already borrowed the value s immutably.
    let r3 = &mut s; // no problem as r1 and r2 are no longer used after this point.
    println!("{}", r3);
}

// eg of dangling references
//
fn main_eg1() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");
    &s
} // s goes out of scope here and is dropped. its memory goes away. so the reference to it will be
  // pointing to nothing. so this will not compile.ERROR
  //
fn no_dangle() -> String {
    let s = String::from("hello");
    s
} // s goes out of scope here but the ownership of s is transferred to the calling function. so this
  // will compile.
  //
fn main_eg2() {
    let x: i32 = 5;
    let p: &i32 = &x;
    println!("the memory address of x is {:p}", p);
}

fn main_eg3() {
    let x: i32 = 5;
    let y: &i32 = &x;

    assert_eq!(5, *y);
    println!("the memory address of x is {:p}", y);
}

// ref can be used to reference the value and not take ownership of it just like &.
// let ref r2 = c ; is equivalent to let r2 = &c;
