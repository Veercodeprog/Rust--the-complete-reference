panic!:
simplest form of error handling in Rust is to use panic! macro
panic! macro is used to stop the execution of the program and print an error message to the console.
In multithreaded programs it will exit the thread in which the panic! occurs , not the whole program.

The simplest error handling mechanism is to use panic. It just prints an error message and
starts unwinding the stack, finally exit the current thread:

if panic occurred in main thread, then the program will be exited.
if in spawned thread, then this thread will be terminated, but the program won't

1.🌟🌟


// FILL the blanks
fn drink(beverage: &str) {
    if beverage == "lemonade" {
        println!("Success!");
        // IMPLEMENT the below code
      panic!()_
     }

    println!("Exercise Failed if printing out this line!");
}

fn main() {
    drink("lemonade");

    println!("Exercise Failed if printing out this line!");
}

2.🌟🌟
// MAKE the code work by fixing all panics
fn main() {
    assert_eq!("abc".as_bytes(), [97, 98, 99]);

    let v: Vec<i32> = vec![1, 2, 3];
    let ele: i32 = v[2];
    // unwrap may panic when get return a None
    let elei: &i32 = v.get(3).unwrap();

    // Sometimes, the compiler is unable to find the overflow errors for you in compile time ,so a panic will occur
    let v = production_rate_per_hour(2);

    // because of the same reason as above, we have to wrap it in a function to make the panic occur
    divide(15, 0);//cant divide by zero

    println!("Success!")
}

fn divide(x:u8, y:u8) {
    println!("{}", x / y)
}

fn production_rate_per_hour(speed: u16) -> f64 {
    let cph: u8 = 221;
    match speed {
        1..=4 => (speed * cph) as f64,
        5..=8 => (speed * cph) as f64 * 0.9,
        9..=10 => (speed * cph) as f64 * 0.77,
        _ => 0 as f64,
    }
}

pub fn working_items_per_minute(speed: u16) -> u32 {
    (production_rate_per_hour(speed) / 60 as f64) as u32
}


unwinding and abort
By default, when a panic occurs, the program starts unwinding, which means Rust walks back up
the stack and cleans up the data from each function it encounters.

But this walk back and clean up is a lot of work. The alternative is to immediately abort
the program without cleaning up.

If in your project you need to make the resulting binary as small as possible,
you can switch from unwinding to aborting by adding below content to Cargo.toml:

[profile.release]
panic = 'abort'
