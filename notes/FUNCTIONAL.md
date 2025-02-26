
# Definitions:
Closures, a function-like construct you can store in a variable
Iterators, a way of processing a series of elements

# Closures
```rs
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}
```

The unwrap_or_else method on Option<T> is defined by the standard library. 
It takes one argument: a closure without any arguments that returns a value T (the same type stored in the Some variant of the Option<T>, in this case ShirtColor).

If the Option<T> is the Some variant, unwrap_or_else returns the value from within the Some. 
If the Option<T> is the None variant, unwrap_or_else calls the closure and returns the value returned by the closure.

We specify the closure expression `|| self.most_stocked()` as the argument to `unwrap_or_else`. 
This is a closure that takes no parameters itself (if the closure had parameters, they would appear between the two vertical bars). 
The body of the closure calls `self.most_stocked()`. 
We’re defining the closure here, and the implementation of `unwrap_or_else` will evaluate the closure later if the result is needed.

An interesting aspect here is that we’ve passed a closure that calls self.most_stocked() on the current Inventory instance. 
The standard library didn’t need to know anything about the Inventory or ShirtColor types we defined, or the logic we want to use in this scenario. 
The closure captures an immutable reference to the self Inventory instance and passes it with the code we specify to the unwrap_or_else method. 
Functions, on the other hand, are not able to capture their environment in this way.

# Closures and Functions
Closures don’t usually require you to annotate the types of the parameters or the return value like fn functions do.
Type annotations are required on functions because the types are part of an explicit interface exposed to your users.
Closures, on the other hand, aren’t used in an exposed interface like this: they’re stored in variables and used without naming them and exposing them to users of our library.

Closures are typically short and relevant only within a narrow context rather than in any arbitrary scenario. 
Within these limited contexts, the compiler can infer the types of the parameters and the return type, similar to how it’s able to infer the types of most variables (there are rare cases where the compiler needs closure type annotations too).

## Closure Type Annotations

```rs
let expensive_closure = |num: u32| -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
};
```

```rs
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```

```rs
let example_closure = |x| x;

let s = example_closure(String::from("hello"));
let n = example_closure(5);
```
With no type annotations, String will be infrerred and the above code will not compile for an number.

Closures can capture values from their environment in three ways, which directly map to the three ways a function can take a parameter: borrowing immutably, borrowing mutably, and taking ownership. 
The closure will decide which of these to use based on what the body of the function does with the captured values.

```rs
fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");
}
```
*Capuring of a immutable reference.*

```rs
fn main() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("After calling closure: {list:?}");
}
```
*Capturing of a mutable reference.*

NOTE: that there’s no longer a println! between the definition and the call of the borrows_mutably closure: when borrows_mutably is defined, it captures a mutable reference to list. 
We don’t use the closure again after the closure is called, so the mutable borrow ends. 
Between the closure definition and the closure call, an immutable borrow to print isn’t allowed because no other borrows are allowed when there’s a mutable borrow.

```rs
use std::thread;

fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();
}
```
*Using **move** to force the closure for a thread to take ownership of list.*

## Fn Traits
Closures will automatically implement one, two, or all three of these Fn traits, in an additive fashion, depending on how the closure’s body handles the values:
- **FnOnce** applies to closures that can be called once. All closures implement at least this trait, because all closures can be called. A closure that moves captured values out of its body will only implement FnOnce and none of the other Fn traits, because it can only be called once.
- **FnMut** applies to closures that don’t move captured values out of their body, but that might mutate the captured values. These closures can be called more than once.
- **Fn** applies to closures that don’t move captured values out of their body and that don’t mutate captured values, as well as closures that capture nothing from their environment. These closures can be called more than once without mutating their environment, which is important in cases such as calling a closure multiple times concurrently.