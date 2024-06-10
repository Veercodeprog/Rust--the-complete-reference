// flow of a program is a  concept that refers to ability  to control the order in which
// statements or instructions are executed in a program.
//
// conditional statements:
// 1. if
// 2. if else
// 3. match
//
// Loops:
// for/ while / loop
// continue / break
//
fn main() {
    for n in 1..100 {
        if n == 100 {
            panic!("NEVER LET THIS RUN！");
        }
    }
}
// enumerate method is used to iterate over a collection and return a tuple containing the index
// and a reference to the element.
//
// fn main() {
//   let a:[i32;5] = [1,2,3,4,5];
//   for (i,v) in a.iter().enumerate() {
//    println!("index: {} value: {}", i, v);
//    println!(" the {}th element is {}", i+1,v);
//    }
//    }
//
//
//    we use loop as a way to create an infinite loop.
//    if loop {
//
//    }
//
//    loop is an expression , so we can use it with break to return a value.
//
//    fn main() {
//    let mut counter = 0;
//    let result = loop {
//    counter += 1;
//    if counter == 10 {
//    break counter * 2;
//    }
//    };
//    assert_eq!(result, 20);
//    }
//
//    🌟🌟🌟 It's possible to break or continue outer loops when dealing with nested loops.
//    In these cases, the loops must be annotated with some 'label, and the label must be
//    passed to the break/continue statement.
//
//
// Fill in the blank
// fn main() {
//     let mut count = 0;
//     'outer: loop {
//         'inner1: loop {
//             if count >= 20 {
//                 // This would break only the inner1 loop
//                 break 'inner1; // `break` is also works.
//             }
//             count += 2;
//         }
//
//         count += 5;
//
//         'inner2: loop {
//             if count >= 30 {
//                 // This breaks the outer loop
//                 break 'outer;
//             }
//
//             // This will continue the outer loop
//             continue 'outer;
//         }
//     }
//
//     assert!(count == __);
//
//     println!("Success!");
// }
