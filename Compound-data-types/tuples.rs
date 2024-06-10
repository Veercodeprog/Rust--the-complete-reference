// tuple
// way to store related data in a single variable.
// collection of values of different types grouped together as a single compound value.
// stored as a fixed size contigous block in the stack memory.
// signature of tuple is (T1, T2, T3, ... Tn) where T1, T2, T3, ... Tn are the types of the
// elements.

// properties of tuple:
// 1. tuples can store values of different data types.
// 2. long tuples(more than 12 elements) can be difficult to work with and cannot be printed or accessed easily.
// 3. tuples can be used as function arguments and return values.
// 4. tuples are immutable once declared.
//
//
//
// // eg of tuples can be used as function arguments and return values.
fn main() {
    let (x, y) = sum_multiply(3, 2);
    asser_eq!(x, 5);
    assert_eq!(y, 6);
    println!("success");
}

fn sum_multiply(nums: (i32, i32)) -> (i32, i32) {
    (nums.o + nums.1, nums.0 * nums.1)
}
