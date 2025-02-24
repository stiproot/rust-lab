Rust’s built-in tuple and array types are stored on the stack, as long as their elements are also stack-allocated types (i.e., types with a known, fixed size at compile time).

Breakdown:
Tuples: Stored on the stack if all elements have a known, fixed size (e.g., (i32, bool, f64)).
Arrays: Stored on the stack because their size is fixed at compile time (e.g., [i32; 5]).

Example (Stored on Stack):
```rs
fn main() {
    let tup: (i32, f64, bool) = (42, 3.14, true);
    let arr: [u8; 4] = [1, 2, 3, 4];

    println!("{:?} {:?}", tup, arr);
}
```

If a tuple or array contains heap-allocated types (like String or Vec<T>), the tuple itself is still on the stack, but the heap-allocated elements are stored on the heap.

Example:
```rs
fn main() {
    let tup = (42, String::from("Hello")); // The tuple is on the stack, but the String is on the heap.
    let arr = [vec![1, 2, 3], vec![4, 5, 6]]; // The array is on the stack, but the Vecs are on the heap.

    println!("{:?} {:?}", tup, arr);
}
```

Three collections that are used very often in Rust programs:

- A vector allows you to store a variable number of values next to each other.
- A string is a collection of characters.
- A hash map allows you to associate a value with a specific key. It’s a particular implementation of the more general data structure called a map.

# Vectors
```rs
let v: Vec<i32> = Vec::new();
let v2 = vec![1, 2, 3];
let v3: Vec<u8> = vec![1, 2, 3];
```

```rs
let mut v = Vec::new();

v.push(1);
v.push(2);
```

```rs
let v = vec![1, 2, 3, 4, 5];

let second: &i32 = &v[1];

let second: Option<&i32> = v.get(1);
match second {
  Some(second) => println!("The second element is {second}"),
  None => println!("There is no second element"),
}
```

```rs
let v = vec![1, 2, 3, 4, 5];

for i in &v: {
  println!("{i}");
}
```

```rs
let mut v = vec![1, 2, 3, 4, 5];

for i in &mut v: {
  *i += 1;
}
```

```rs
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
```

# Strings
The String type, which is provided by Rust’s standard library rather than coded into the core language, is a growable, mutable, owned, UTF-8 encoded string type.
When Rustaceans refer to “strings” in Rust, they might be referring to either the String or the string slice &str types, not just one of those types. 
Although this section is largely about String, both types are used heavily in Rust’s standard library, and both String and string slices are UTF-8 encoded.


Many of the same operations available with Vec<T> are available with String as well because String is actually implemented as a wrapper around a vector of bytes with some extra guarantees, restrictions, and capabilities.
```rs
let mut s = String::new();
```

```rs
let data = "initial contents";

let s = data.to_string();

// the method also works on a literal directly:
let s = "initial contents".to_string();
```

Is the same as...

```rs
let s = String::from("initial contents");
```

```rs
let mut s = String::from("lo");
s.push('l');
```

```rs
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
```

```rs
for c in "Зд".chars() {
    println!("{c}");
}

for b in "Зд".bytes() {
    println!("{b}");
}
```

# Hash Maps

```rs
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Green"), 20);

let team_name = String::from("Blue");

let score = scores.get(&team_name).copied().unwrap_or(0);

for (key, value) in &scores {
  println!("key: {key}, value: {value}");
}

```

For types that implement the Copy trait, like i32, the values are copied into the hash map. 
For owned values like String, the values will be moved and the hash map will be the owner of those values.
```rs
use std::collections::HashMap;

let field_name = String::from("Favorite color");
let field_value = String::from("Blue");

let mut map = HashMap::new();
map.insert(field_name, field_value);
// field_name and field_value are invalid at this point, try using them and
// see what compiler error you get!
```

```rs
use std::collections::HashMap;

let text = "hello world wonderful world";

let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}

println!("{map:?}");
```