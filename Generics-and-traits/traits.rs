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
