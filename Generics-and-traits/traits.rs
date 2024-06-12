// Traits:
// set of methods that can be implemented for multiple types in order to provide a common
// functionality and behavior between them.
// traits consists of method signature only , which then have to be implemented by the target type.
// Similar to classes in other languages , not quite the same though.
//  Defines shared behavior in an abstract way.
//
//
//  trait Animal {
//   fn sound(&self) -> String;
//   }
//   struct Sheep;
//   struct Cow;
//
//   impl Animal for Sheep {
//   fn sound(&self) -> String {
//   String::from("baaaaah")
//   }
//   }
//
//   impl Animal for Cow {
//   fn sound(&self) -> String {
//   String::from("moooooo")
//      }
//      }
//
//   in trait we define the signature only.
//   then we implement this signature for every type we want to use it with.
//
//   Derivable trait:
//   Trait that can be automatically implemented for any type
//   (struct or enum) that meets the trait's requirements.
//   called derivable because they can be derived automatically.
//   Most common derivable traits:
//   1. Debug - allows to print the struct or enum using {:?} or {:#?}
//   2. Clone - allows type to be duplicated with .clone() method
//   3. Copy - allows type to be copied implicitly by value rather than moved, without requiring a .clone()
//      method explicitly.
//   4. PartialEq and Eq - allows to compare two values of the type for equality
//
//
//   Traits can be used as parameters in functions:
//
//   pub fn notify(item: &impl Summary) {
//          println!("Breaking news! {}", item.summarize());
//          }
//
//   The function notify() takes as argument any type that implements the Summary trait.
//
//Trait bounds: are declared like generics, after name of the function.use trait bounds if you have
//lots of parameters to avoid this:
//
//pub fn notify(item1: &impl Summary, item2: &impl Summary) {
//  ..
//  }
//
//  use trait bounds instead for above:
//  pub fn notify<T: Summary>(item1: &T, item2: &T) {
//  ..
//      }
//
//
//  Where clause: is used to specify multiple trait bounds.
//
//fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
//
//if we have a function that makes heavy use of trait bounds, we can use a where clause to make the
//code cleaner.
//
//  fn some_function<T, U>(t: &T, u: &U) -> i32
//  where
//  T: Display + Clone,
//  U: Clone + Debug
//  {
//
//
//  Return types that implement traits:
//
//  trait Animal{}
//
//  struct  Dog;
//  struct Cat;
//
//  impl Animal for Dog{}
//  impl Animal for Cat{}
//
//  fn return_dog() -> impl Animal {
//   Dog{}
//   }
//   fn return_cat() -> impl Animal {
//   Cat{}
//   }
//   fn main() {
//   let dog = return_dog();
//   let cat = return_cat();
//   }
//
//   here we have Animal trait,which is implemented for two structs Dog and Cat.
//
//   the two functions return_dog() and return_cat() return Dog and Cat respectively.
//
//   1.🌟🌟
//
// Fill in the two impl blocks to make the code work.
// DON'T modify the code in `main`.
trait Hello {
    fn say_hi(&self) -> String {
        String::from("hi")
    }

    fn say_something(&self) -> String;
}

struct Student {}
impl Hello for Student {
    fn say_something(&self) -> String {
        String::from("I'm a good student")
    }
}
struct Teacher {}
impl Hello for Teacher {
    fn say_hi(&self) -> String {
        String::from("Hi, I'm your new teacher")
    }
    fn say_something(&self) -> String {
        String::from("I'm not a bad teacher")
    }
}

fn main() {
    let s = Student {};
    assert_eq!(s.say_hi(), "hi");
    assert_eq!(s.say_something(), "I'm a good student");

    let t = Teacher {};
    assert_eq!(t.say_hi(), "Hi, I'm your new teacher");
    assert_eq!(t.say_something(), "I'm not a bad teacher");

    println!("Success!");
}

// 2.🌟🌟
//Derive
// The compiler is capable of providing basic implementations for some traits via the #[derive] attribute.
//
// `Centimeters`, a tuple struct that can be compared
#[derive(PartialEq, PartialOrd)] //compiler will automatically implement PartialEq and PartialOrd
                                 //for Centimeters
struct Centimeters(f64);

// `Inches`, a tuple struct that can be printed
#[derive(Debug)] //compiler will automatically implement Debug for Inches
struct Inches(i32);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let &Inches(inches) = self;

        Centimeters(inches as f64 * 2.54)
    }
}

// ADD some attributes to make the code work!
// DON'T modify other code!
#[derive(Debug, PartialEq, PartialOrd)]
struct Seconds(i32);

fn main() {
    let _one_second: Seconds = Seconds(1);

    println!("One second looks like: {:?}", _one_second);
    let _this_is_true = (_one_second == _one_second);
    let _this_is_false = (_one_second > _one_second);

    let foot: Inches = Inches(12);

    println!("One foot equals {:?}", foot);

    let meter: Centimeters = Centimeters(100.0);

    let cmp: &str = if foot.to_centimeters() < meter {
        "smaller"
    } else {
        "bigger"
    };

    println!("One foot is {} than one meter.", cmp);
}

// 3.🌟🌟
// Operator:
// In Rust many of the operators can be overloaded via traits.That is some operators can be used to
// accomplish different tasks depending on the type of their operands or input arguments.This is
// possible because operators are syntactic sugar for method calls..
// For example, the + operator in a + b calls the add method (a.add(b)) on the a object.This add
// method is part of the Add trait.Hence, the + operator can be used on any type that implements
// the Add trait.
//
//
use std::ops;

// Implement fn multiply to make the code work.
// As mentioned above, `+` needs `T` to implement `std::ops::Add` Trait.
// So, what about `*`?  You can find the answer here: https://doc.rust-lang.org/core/ops/
fn multiply<T>(a: T, b: T) -> T
where
    T: std::ops::Mul<Output = T>,
{
    a * b
}

fn main() {
    assert_eq!(6, multiply(2u8, 3u8));
    assert_eq!(5.0, multiply(1.0, 5.0));

    println!("Success!");
}

// 4.🌟🌟
//
// Fix the errors, DON'T modify the code in `main`.
use std::ops;

struct Foo;
struct Bar;

#[derive(Debug, PartialEq)] //automatically implement Debug and PartialEq for FooBar, the debug
                            //trait is used to format a value using the {:?} formatter and PartialEq enables comparison using
                            // == and !=
struct FooBar;
#[derive(Debug, PartialEq)]
struct BarFoo;

// The `std::ops::Add` trait is used to specify the functionality of `+`.
// Here, we make `Add<Bar>` - the trait for addition with a RHS of type `Bar`.
// The following block implements the operation: Foo + Bar = FooBar
impl ops::Add<Bar> for Foo {
    //impl ops::Add<Bar> for Foo: This line specifies that we're implementing the Add trait for Foo
    //where the right-hand side of the + operator is a Bar
    //implementing Add trait for Foo
    type Output = FooBar;

    fn add(self, _rhs: Bar) -> FooBar {
        FooBar
    }
}

impl ops::Sub<Bar> for Foo {
    //implementing Sub trait for Bar
    type Output = BarFoo;

    fn sub(self, _rhs: Bar) -> BarFoo {
        BarFoo
    }
}

fn main() {
    // DON'T modify the code below.
    // You need to derive some trait for FooBar to make it comparable.

    assert_eq!(Foo + Bar, FooBar);
    assert_eq!(Foo - Bar, BarFoo);

    println!("Success!");
}

// 5.🌟🌟
// Use trait as function parameter:
// Instead of a concrete type for the item parameter , we specify the impl keyword and the trait
// name. This parameter accepts any type that implements the specified trait.
//
//
// Implement `fn summary` to make the code work.
// Fix the errors without removing any code line
trait Summary {
    fn summarize(&self) -> String;
}

#[derive(Debug)]
struct Post {
    title: String,
    author: String,
    content: String,
}

impl Summary for Post {
    fn summarize(&self) -> String {
        format!("The author of post {} is {}", self.title, self.author)
    }
}

#[derive(Debug)]
struct Weibo {
    username: String,
    content: String,
}

impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{} published a weibo {}", self.username, self.content)
    }
}

fn main() {
    let post: Post = Post {
        title: "Popular Rust".to_string(),
        author: "Sunface".to_string(),
        content: "Rust is awesome!".to_string(),
    };
    let weibo: Weibo = Weibo {
        username: "sunface".to_string(),
        content: "Weibo seems to be worse than Tweet".to_string(),
    };

    summary(&post);
    summary(&weibo);

    println!("{:?}", post);
    println!("{:?}", weibo);
}

// Implement `fn summary` below.
fn summary<T: summary>(a: &T) {
    //we use &impl and not impl because we want to pass a reference to
    //the function and not the value itself.
    println!("{}", a.summarize());
}

// 6.🌟🌟
// Returning Types that Implement Traits:
//
// we can also use  the impl Trait syntax in the return position to return a value of some type
// that implements a trait, as shown in the following example:
// However, we can only use impl train when returning a single type, Use a trait object if you need
// to return multiple types.
//
struct Sheep {}
struct Cow {}

trait Animal {
    fn noise(&self) -> String;
}

impl Animal for Sheep {
    fn noise(&self) -> String {
        "baaaaah!".to_string()
    }
}

impl Animal for Cow {
    fn noise(&self) -> String {
        "moooooo!".to_string()
    }
}

// Returns some struct that implements Animal, but we don't know which one at compile time.
// FIX the errors here, you can make a fake random, or you can use trait object.
fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}

fn main() {
    let random_number = 0.234;
    let animal: Box<dyn Animal> = random_animal(random_number);
    println!(
        "You've randomly chosen an animal, and it says {}",
        animal.noise()
    );
}

//Trait objects:
//Using 'impl Trait' doesn't work when returning multiple types, because the compiler needs to know
//the size of the return type at compile time. In such cases, we can use trait objects.
//Different implementation of a trait probably uses different memory size, so the compiler can't
//decide the size of the return type at compile time but it is required to know the size of the
//return type at compile time.
//To solve this problem, we can use trait objects, which are a way to abstract over types that
//implement a trait.
//
//A trait object is essentially a pointer to any type that implements the given trait,where the
//precise type can only be known at runtime.
//
//Dynamic trait object:
//as the trait object is behind a pointer, the size of the pointer is known at compile time, which
//is usize(size of pointer).
trait Animal {}
struct Dog;
struct Cat;

impl Animal for Dog {}
impl Animal for Cat {}
fn return_animal(s: &str) -> &dyn Animal {
    // &dyn stands for dynamic
    if s == "dog" {
        &Dog {}
    } else {
        &Cat {}
    }

    //match s{
    //"dog" => &Dog{},
    //"cat" => &Cat{},
    //_ => panic!("invalid animal")
    //}
}

fn main() {
    let dog = return_animal("dog");
    let cat = return_animal("cat");
}

//Static Dispatch:
//resolve the method calls at compile time, which is faster than dynamic dispatch.
//Compiler generates function code for each concrete type that implements the trait.
//calls appropriate function code based on the concrete types.
//Faster and more efficient than dynamic dispatch.but doesnt provide great flexibility.
// Static dispatch is a mechanism where compiler decides which implementation of a trait will be
// used at a compile time.IT involves compiler generating code for each concrete type that
// implements the trait and calling the appropriate function code based on the concrete types.
// it is faster because the compiler knows the exact type at compile time and doesn't need runtime
// type checking.
//
// eg
// trait Animal{
// fn say_hi(&self);
// }
//
// struct Dog;
// struct Cat;
// impl Animal for Dog{
// fn say_hi(&self){
// println!("woff");
// }
// impl Animal for Cat{
// fn say_hi(&self){
// println!("meow");
// }
// }
// fn main(){
// let dog = Dog{};
// let cat = Cat{};
// dog.say_hi();
// cat.say_hi();
// }
//
// on compiling the code , the compiler generates the code for each concrete type that implements
// the trait and calls the appropriate function code based on the concrete types like below:-
// compiler generated code:
// struct Dog;
// struct Cat;
// impl Dog{
// fn say_hi(&self){
// println!("woff");
// }
// impl Cat{
// fn say_hi(&self){
// println!("meow");
// }
// fn main(){
// let dog = Dog{};
// let cat = Cat{};
// dog.say_hi();
// cat.say_hi();
// }
//
// Dynamic Dispatch:
// Dynamic dispatch in rust is a mechanism that allows program to decide at runtime which method
// implementation to call.This is achieved by using trait objects.Trait object allows you to store
// different types of values that implements the same trait , enabling polymorphism.
// works by creating a refernce or smart pointer to a trait object using &dyn or Box<dyn> syntax.
// when trait object is created , compiler will build a vtable for the trait object, which contains
// pointers to the method implementations of the trait.
// vtable : is a table that contains a pointer to the each method implementation of the trait for
// the specific type of the object that reference points to.
// compiler will do a lookup in the vtable to find the correct method implementation to call for
// which type that implements the given trait.
// this lookup will cause overhead but allows more flexibility in code.
//
// eg :
//
// trait Animal{
// fn noise(&self);
// }
// struct Dog;
//  struct Cat;
//  impl Animal for Dog{
//  fn noise(&self){
//  println!("woff");
//  }
//  }
//  impl Animal for Cat{
//  fn noise(&self){
//  println!("meow");
//  }
//  }
//  fn random_animal(random_number: u8) -> Box<dyn Animal>{
//  if random_number < 128{
//  Box::new(Dog{})
//  }else{
//  Box::new(Cat{})
//  }
//  }
//
//  fn main(){
//  let random_number = 100;
//  let animal = random_animal(random_number);
//  animal.noise();
//  }
//
//  Compiler can not know the size of the return type at compile time, so we use Box<dyn> to return
//  a trait object.
//  Box<dyn> is a smart pointer that allows you to store a value on the heap and return a reference
//  to it.
//  then noise() method gets called on the returned trait object, the compiler will look up the
//  correct method implementation in the vtable and call the appropriate method.this resolves at
//  runtime.
//
//
//Box: smart pointer that allows you to store a value on the heap rather than stack and return a reference to it.
//Use Box when you have a type whose size can't be known at compile time and you want to use a
//value of that type in a context that requires an exact size.
//Returns a pointer to the data stored on the heap or heap allocated data.
//
//& vs Box:
//Memory: Box allocated data on the heap and owns it , also responsible for deallocating when value
//goes out of scope.& only points to the value or data already in memory.
//
//Lifetime:Box can be passed accross scopes ,reference has limited lifetime.
//Box can be cloned , reference can't be cloned.
//Box can be used in pattern matching.
//
//
//Trait Bound:
//the impl keyword works for straitforward cases, but it has some limitations and is actually
//a syntactic sugar for a longer form called trait bound.
//
//when working with generics , the type parameter often must use traits as bounds to stipulate
//what functionality a type implements.
//
//7.🌟🌟
fn main() {
    assert_eq!(sum(1, 2), 3);
}

// Implement `fn sum` with trait bound in two ways.
fn sum<T: std::ops::Add<Output = T>>(x: T, y: T) -> T {
    x + y
}

//8.🌟🌟
//
// FIX the errors.
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: std::fmt::Debug + PartialOrd + PartialEq> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {:?}", self.x);
        } else {
            println!("The largest member is y = {:?}", self.y);
        }
    }
}

struct Unit(i32);

#[derive(Debug, PartialOrd, PartialEq)] // PartialOrd and PartialEq are derivable traits.PartiaOrd
                                        // is used to compare two values and PartialEq is used to compare
                                        // two values for equality.
fn main() {
    // let pair: Pair<Unit> = Pair {
    //     x: Unit(1),
    //     y: Unit(3),
    // };

    //for associated function
    let pair: Pait<Unit> = Pair::new(Unit(1), Unit(3));
    pair.cmp_display();
}


9.🌟🌟

// Fill in the blanks to make it work
fn example1() {
    // `T: Trait` is the commonly used way.
    // `T: Fn(u32) -> u32` specifies that we can only pass a closure to `T`.
    struct Cacher<T: Fn(u32) -> u32> {
        calculation: T,
        value: Option<u32>,
    }

    impl<T: Fn(u32) -> u32> Cacher<T> {
        fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation,
                value: None,
            }
        }

        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                },
            }
        }
    }

    let mut cacher = Cacher::new(|x| x+1);
    assert_eq!(cacher.value(10), __);
    assert_eq!(cacher.value(15), __);
}


fn example2() {
    // We can also use `where` to construct `T`
    struct Cacher<T>
        where T: Fn(u32) -> u32,
    {
        calculation: T,
        value: Option<u32>,
    }

    impl<T> Cacher<T>
        where T: Fn(u32) -> u32,
    {
        fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation,
                value: None,
            }
        }

        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                },
            }
        }
    }

    let mut cacher = Cacher::new(|x| x+1);
    assert_eq!(cacher.value(20), __);
    assert_eq!(cacher.value(25), __);
}



fn main() {
    example1();
    example2();

    println!("Success!");
}



ASSOCIATED TYPES:
1.) Allow to specify that is associated with the traits.
2.) When implementing a trait for a specific type , we have to specify the concrete type.
3.) Concrete type is specific , non-generic type that is fully defined and can be used to create instances.
4.)associated type is basically a type placeholder that the trait methods can use in their signatures.
5.)similar to generic types but they are more flexible because they allow a trait to have different associated types
for differnt implementing types.

Associated type eg:

trait MyTrait {
   type MyType;  //associated type
   fn get_my_type(&self) -> Self::MyType;
}

struct MyStruct{}
impl MyTrait for MyStruct{
   type MyType = i32;   //implenting the trait so we have to specify the concrete type.
   fn get_my_type(&self) -> Self::MyType{
   10
   }
}


Trait object excercises:
1.🌟🌟

trait Bird {
    fn quack(&self) -> String;
}

struct Duck;
impl Duck {
    fn swim(&self) {
        println!("Look, the duck is swimming")
    }
}
struct Swan;
impl Swan {
    fn fly(&self) {
        println!("Look, the duck.. oh sorry, the swan is flying")
    }
}

impl Bird for Duck {
    fn quack(&self) -> String{
        "duck duck".to_string()
    }
}

impl Bird for Swan {
    fn quack(&self) -> String{
        "swan swan".to_string()
    }
}

fn main() {
    // FILL in the blank.
    let duck: Duck = Duck;
    duck.swim();

    let bird: Box<dyn Bird>  = hatch_a_bird(2);
    // This bird has forgotten how to swim, so below line will cause an error.
    // bird.swim();
    // But it can quak.
    assert_eq!(bird.quack(), "duck duck");

    let bird: Box<dyn Bird>  = hatch_a_bird(1);
    // This bird has forgotten how to fly, so below line will cause an error.
    // bird.fly();
    // But it can quak too.
    assert_eq!(bird.quack(), "swan swan");

    println!("Success!");
}

// IMPLEMENT this function.
fn hatch_a_bird(species:u8) -> Box<dyn Bird> {
  match species{
    1 => Box::new(Swan{}),
    2 => Box::new(Duck{}),
    _ => panic!("invalid species")

    }
}

2.🌟🌟
Array with trait object:
trait Bird {
    fn quack(&self);
}

struct Duck;
impl Duck {
    fn fly(&self) {
        println!("Look, the duck is flying")
    }
}
struct Swan;
impl Swan {
    fn fly(&self) {
        println!("Look, the duck.. oh sorry, the swan is flying")
    }
}

impl Bird for Duck {
    fn quack(&self) {
        println!("{}", "duck duck");
    }
}

impl Bird for Swan {
    fn quack(&self) {
        println!("{}", "swan swan");
    }
}

fn main() {
    // FILL in the blank to make the code work.
    let birds [&dyn Bird;2 ] = [&Duck, &Swan ] ; //usize i.e. size of pointer  // &dyn Bird or Box<dyn Bird> points to a bird trait.

    for bird in birds {
        bird.quack();
        // When duck and swan turn into Birds, they all forgot how to fly, only remember how to quack.
        // So, the code below will cause an error.
        // bird.fly();
    }
}

3.🌟🌟
&dyn and Box<dyn>:


// FILL in the blanks.
trait Draw {
    fn draw(&self) -> String;
}

impl Draw for u8 {
    fn draw(&self) -> String {
        format!("u8: {}", self)
    }
}

impl Draw for f64 {
    fn draw(&self) -> String {
        format!("f64: {}", self)
    }
}

fn main() {
    let x:f64 = 1.1f64;
    let y:u8 = 8u8;

    // Draw x.
    draw_with_box(Box::new(x));

    // Draw y.
    draw_with_ref(&y);

    println!("Success!");
}

fn draw_with_box(x: Box<dyn Draw>) {
    x.draw();
}

fn draw_with_ref(x: &dyn Draw) {
    x.draw();
}

4.🌟🌟

trait Foo {
    fn method(&self) -> String;
}

impl Foo for u8 { //we can implement the trait for the types that are defined in the standard
    //library.
    fn method(&self) -> String { format!("u8: {}", *self) }
}

impl Foo for String {
    fn method(&self) -> String { format!("string: {}", *self) }
}

// IMPLEMENT below with generics.
fn static_dispatch<T: Foo>(x:T) -> String{
x.method()
}


// Implement below with trait objects.
fn dynamic_dispatch(x: &dyn Foo) -> String{
x.method()
}

fn main() {
    let x:u8 = 5u8;
    let y:String = "Hello".to_string();

    static_dispatch(x);
    dynamic_dispatch(&y);

    println!("Success!");
}

5.)Object SAFE:
We can only make object safe traits into trait objects.A trait is object safe if all the methods defined
in the trait  have the following properties:
- The return type isn't Self.
- There are no generic type parameters.


// Use at least two approaches to make it work.
// DON'T add/remove any code line.
trait MyTrait {
    fn f(&self) -> Box<dyn MyTrait>;
}

impl MyTrait for u32 {
    fn f(&self) -> Box<dyn MyTrait> { Box::new(42) }
}

impl MyTrait for String {
    fn f(&self) -> Box<dyn MyTrait> { Box::new(self.clone()) }
}

fn my_function(x: Box<dyn MyTrait>)-> Box<dyn MyTrait> {
    x.f()
}

fn main() {
    my_function(Box::new(13_u32));
    my_function(Box::new(String::from("abc")));

    println!("Success!");
}
