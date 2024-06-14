HASHMAP:
where vectors store values by an index, hashmaps store values by a key.
data structure that stores key-value pairs.
Allocated on the heap as it is dynamically sized.
allows for efficient lookups, insertions, and deletions.
each key is hashed to unique index in the underlying array.
By default, HashMap uses a hashing algorithm selected to provide resistance against HashDoS attacks.

The default hashing algorithm is currently SipHash 1-3, though this is subject to change at any point in the future.

1. 🌟🌟

// FILL in the blanks and FIX the errors
use std::collections::HashMap;
fn main() {
    let mut scores: HashMap<&str, i32> = HashMap::new();
    scores.insert("Sunface", 98);
    scores.insert("Daniel", 95);
    scores.insert("Ashley", 69.0);
    scores.insert("Katie", 58);

    // Get returns an Option<&V>
    let score: Option<&i32> = scores.get("Sunface");
    assert_eq!(score, Some(&98));

    if scores.contains_key("Daniel") {
        // Indexing returns a value V
        let score: i32 = scores["Daniel"];
        assert_eq!(score, 95)
        scores.remove("Daniel");
    }

    assert_eq!(scores.len(), 3);

    for (name, score) in scores {
        println!("The score of {} is {}", name, score);
    }
}


2. 🌟🌟

use std::collections::HashMap;
fn main() {
    let teams: [(&str, i32);3] = [
        ("Chinese Team", 100),
        ("American Team", 10),
        ("France Team", 50),
    ];

    let mut teams_map1: HashMap<&str, i32> = HashMap::new();
    for team in &teams {
        teams_map1.insert(team.0, team.1);
    }

    // IMPLEMENT team_map2 in two ways
    // Tips: one of the approaches is to use `collect` method
    let teams_map2: HashMap<&str, i32> = HashMap::from(teams);  // we can also use
    // teams.into_iter().collect()

    assert_eq!(teams_map1, teams_map2);

    println!("Success!");
}

3.) 🌟🌟

// FILL in the blanks
use std::collections::HashMap;
fn main() {
    // Type inference lets us omit an explicit type signature (which
    // would be `HashMap<&str, u8>` in this example).
    let mut player_stats = HashMap::new();

    // Insert a key only if it doesn't already exist
    player_stats.entry("health").or_insert(100);

    assert_eq!(player_stats["health"], 100);

    // Insert a key using a function that provides a new value only if it
    // doesn't already exist
    player_stats.entry("health").or_insert_with(random_stat_buff);
    assert_eq!(player_stats["health"], 100);

    // Ensures a value is in the entry by inserting the default if empty, and returns
    // a mutable reference to the value in the entry.
    let health: &mut u8 = player_stats.entry("health").or_insert(50); //if value is not present, it will
    //insert 50 and return a mutable reference to the value
    assert_eq!(health, &100);
    *health -= 50;
    assert_eq!(*health, 50);

    println!("Success!");
}

fn random_stat_buff() -> u8 {
    // Could actually return some random value here - let's just return
    // some fixed value for now
    42
}


Requirements of HashMap Key:
Any type that implements the Eq and Hash traits can be a key in HashMap..This includes:
bool
int , uint, and all variations thereof
String and &str

Note that f32 and f64 do not implement Hash, so they cannot be used as keys ,likely because floating point calculations
are not deterministic and could lead to bugs.

All collection classes implement Eq and Hash if their contained type also implements Eq and Hash.
for eg Vec<T> implements Eq and Hash if T implements Eq and Hash.:w

4.) 🌟🌟

// FIX the errors
// Tips: `derive` is usually a good way to implement some common used traits
use std::collections::HashMap;
#[derive(Debug, Eq, PartialEq, Hash)]
struct Viking {
    name: String,
    country: String,
}

impl Viking {
    /// Creates a new Viking.
    fn new(name: &str, country: &str) -> Viking {
        Viking {
            name: name.to_string(),
            country: country.to_string(),
        }
    }
}

fn main() {
    // Use a HashMap to store the vikings' health points.
    let vikings<Viking, i32> = HashMap::from([
        (Viking::new("Einar", "Norway"), 25),
        (Viking::new("Olaf", "Denmark"), 24),
        (Viking::new("Harald", "Iceland"), 12),
    ]);

    // Use derived implementation to print the status of the vikings.
    for (viking, health) in &vikings {
        println!("{:?} has {} hp", viking, health);
    }
}


//Capacity:
//Like vectors, HashMaps are growable, but HashMaps can also shrink themselves when
//they have excess space. You can create a HashMap with a certain starting capacity using
//HashMap::with_capacity(uint), or use HashMap::new() to get a HashMap with a default initial capacity (recommended).
//
//eg:
//
use std::collections::HashMap;
fn main() {
    let mut map: HashMap<i32, i32> = HashMap::with_capacity(100);
    map.insert(1, 2);
    map.insert(3, 4);
    // Indeed ,the capacity of HashMap is not 100, so we can't compare the equality here.
    assert!(map.capacity() >= 100);

    // Shrinks the capacity of the map with a lower limit. It will drop
    // down no lower than the supplied limit while maintaining the internal rules
    // and possibly leaving some space in accordance with the resize policy.

    map.shrink_to(50);
    assert!(map.capacity() >= 50);

    // Shrinks the capacity of the map as much as possible. It will drop
    // down as much as possible while maintaining the internal rules
    // and possibly leaving some space in accordance with the resize policy.
    map.shrink_to_fit();
    assert!(map.capacity() >= 2);
    println!("Success!");
}

Ownership in hashmaps:
For types that implement the Copy trait, like i32 , the values are copied into HashMap.
For owned values like String, the values will be moved and HashMap will be the owner of those values.
// FIX the errors with least changes
// DON'T remove any code line
use std::collections::HashMap;
fn main() {
  let v1: i32 = 10;
  let mut m1: HashMap<i32, i32> = HashMap::new();
  m1.insert(v1, v1);
  println!("v1 is still usable after inserting to hashmap : {}", v1);

  let v2: String = "hello".to_string();
  let mut m2: HashMap<&str, i32> = HashMap::new();
  // if we use v2 instead of &v2 then  Ownership moved here
  m2.insert(&v2, v1);

  assert_eq!(v2, "hello");

  println!("Success!");
}

