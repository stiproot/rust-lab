A pointer is a general concept for a variable that contains an address in memory.
The most common kind of pointer in Rust is a reference.
References are indicated by the `&` symbol and borrow the value they point to. 
They don’t have any special capabilities other than referring to data, and have no overhead.

`Smart pointers`, on the other hand, are data structures that act like a pointer but also have additional metadata and capabilities.

Rust, with its concept of ownership and borrowing, has an additional difference between references and smart pointers: while references only borrow data, in many cases, **smart pointers own the data they point to**.

`String` and `Vec<T>` are examples of smart pointers.
Both own some memory and allow you to manipulate it. 
They also have metadata and extra capabilities or guarantees. `String`, for example, stores its capacity as metadata and has the extra ability to ensure its data will always be valid UTF-8.

Smart pointers are usually implemented using structs. Unlike an ordinary struct, smart pointers implement the `Deref` and `Drop` traits.

The `Deref` trait allows an instance of the smart pointer struct to behave like a reference so that you can write you code to work with either smart pointers or references.
The `Drop` trait allows you to customize the code that's run when an instance of the smart pointer goes out of scope.

Smart pointers is a design pattern. Therefore there are many implementations of them.
The most common in the standard library are:
- `Box<T>` for allocating values on the heap
- `Rc<T>`, a reference counting type that enables multiple ownership
- `Ref<T>` and `RefMut<T>`, accessed through `RefCell<T>`, a type that enforces the borrowing rules at runtime instead of compile time

# Box<T>
The most straightforward smart pointer is a box, whose type is written Box<T>. 
Boxes allow you to store data on the heap rather than the stack. What remains on the stack is the pointer to the heap data

Used most often in these situations:
1. When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size
2. When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so
3. When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type

```rs
fn main() {
    let b = Box::new(5);
    println!("b = {b}");
}
```
*storing i32 value on the heap using box*
Just like any owned value, when a box goes out of scope, as b does at the end of main, it will be deallocated. 
The deallocation happens both for the box (stored on the stack) and the data it points to (stored on the heap).

# Cons List (Recursive Types)
Because a Box<T> is a pointer, Rust always knows how much space a Box<T> needs: a pointer’s size doesn’t change based on the amount of data it’s pointing to. 
This means we can put a Box<T> inside the Cons variant instead of another List value directly. 
The Box<T> will point to the next List value that will be on the heap rather than inside the Cons variant. 
Conceptually, we still have a list, created with lists holding other lists, but this implementation is now more like placing the items next to one another rather than inside one another.
```rs
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
```

The Box<T> type is a smart pointer because it implements the Deref trait, which allows Box<T> values to be treated like references.
When a Box<T> value goes out of scope, the heap data that the box is pointing to is cleaned up as well because of the Drop trait implementation.

The following:
```rs
fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

Could be rewritten as:
```rs
fn main() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

# Deref Coercions with Functions and Methods
Deref coercion converts a reference to a type that implements the `Deref` trait into a reference to another type.
For example, deref coercion can convert `&String` to `&str` because String implements the `Deref` trait such that it returns `&str`.
Deref coercion is a convenience Rust performs on arguments to functions and methods, and works only on types that implement the Deref trait.
**It happens automatically when we pass a reference to a particular type’s value as an argument to a function or method that doesn’t match the parameter type in the function or method definition.**
A sequence of calls to the deref method converts the type we provided into the type the parameter needs.

Deref coercion makes it possible to call hello with a reference to a value of type MyBox<String>
```rs
fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}
```

If Rust didn’t implement deref coercion, we would have to write the above code to call hello with a value of type &MyBox<String>.
```rs
fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]);
}
```

# How Deref Coercion Interacts with Mutability
Similar to how you use the `Deref` trait to override the * operator on immutable references, you can use the `DerefMut` trait to override the * operator on mutable references.

Rust does deref coercion when it finds types and trait implementations in three cases:
- From &T to &U when T: Deref<Target=U>
- From &mut T to &mut U when T: DerefMut<Target=U>
- From &mut T to &U when T: Deref<Target=U>

The first two cases are the same as each other except that the second implements mutability. 
The first case states that if you have a &T, and T implements Deref to some type U, you can get a &U transparently. 
The second case states that the same deref coercion happens for mutable references.

The third case is trickier: Rust will also coerce a mutable reference to an immutable one. 
But the reverse is not possible: immutable references will never coerce to mutable references. 
Because of the borrowing rules, if you have a mutable reference, that mutable reference must be the only reference to that data (otherwise, the program wouldn’t compile). 
Converting one mutable reference to one immutable reference will never break the borrowing rules. 
Converting an immutable reference to a mutable reference would require that the initial immutable reference is the only immutable reference to that data, but the borrowing rules don’t guarantee that. 
Therefore, Rust can’t make the assumption that converting an immutable reference to a mutable reference is possible.

# Drop Trait
Drop lets you customize what happens when a value is about to go out of scope. 
You can provide an implementation for the Drop trait on any type, and that code can be used to release resources like files or network connections.

```rs
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
}
```
*Rust automatically called drop for us when our instances went out of scope, calling the code we specified.* 

Occasionally, you might want to clean up a value early. 
One example is when using smart pointers that manage locks: you might want to force the drop method that releases the lock so that other code in the same scope can acquire the lock. 
Rust doesn’t let you call the `Drop` trait’s drop method manually; instead you have to call the `std::mem::drop` function provided by the standard library if you want to force a value to be dropped before the end of its scope.


The std::mem::drop function is different from the drop method in the Drop trait. 
We call it by passing as an argument the value we want to force drop. 
```rs
fn main() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}
```
*The text Dropping CustomSmartPointer with data `some data`! is printed between the CustomSmartPointer created. and CustomSmartPointer dropped before the end of main. text, showing that the drop method code is called to drop c at that point.*

# Rc<T>
In the majority of cases, ownership is clear: you know exactly which variable owns a given value. 
However, there are cases when a single value might have multiple owners. 
For example, in graph data structures, multiple edges might point to the same node, and that node is conceptually owned by all of the edges that point to it.
`Rc<T>` is only for use in single-threaded scenarios.

You have to enable multiple ownership explicitly by using the Rust type `Rc<T>`, which is an abbreviation for reference counting.
The `Rc<T>` type keeps track of the number of references to a value to determine whether or not the value is still in use.

```rs
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}
```
*Cons list refactored to cater for shared lists, only possible with `Rc`*

The implementation of `Rc::clone` doesn’t make a deep copy of all the data like most types’ implementations of clone do. 
The call to `Rc::clone` only increments the reference count, which doesn’t take much time. 

```rs
fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
```

# RefCell<T>
The *Interior mutability* is a design pattern in Rust that allows you to mutate data even when there are immutable references to that data; normally this action is disallowed by tghe borrowing rules.
To mutate data, the pattern uses `unsafe` code inside a data structure to bend Rust's usual rules that govern mutation and borrowing.
`RefCell<T>` is only for use in single-threaded scenarios.

Recall the borrowing rules:
- At any given time, you can have either (but not both) one mutable reference or any number of immutable references.
- References must always be valid.

With references and `Box<T>`, the borrowing rules’ invariants are enforced at compile time. 
With `RefCell<T>`, these invariants are enforced at runtime. 
With references, if you break these rules, you’ll get a compiler error. 
With `RefCell<T>`, if you break these rules, your program will panic and exit.

The `RefCell<T>` type is useful when you’re sure your code follows the borrowing rules but the compiler is unable to understand and guarantee that.


Here is a recap of the reasons to choose `Box<T>`, `Rc<T>`, or `RefCell<T>`:
- `Rc<T>` enables multiple owners of the same data; `Box<T>` and `RefCell<T>` have single owners.
- `Box<T>` allows immutable or mutable borrows checked at compile time; `Rc<T>` allows only immutable borrows checked at compile time; `RefCell<T>` allows immutable or mutable borrows checked at runtime.
- Because `RefCell<T>` allows mutable borrows checked at runtime, you can mutate the value inside the `RefCell<T>` even when the `RefCell<T>` is immutable.

Here is an example of a use-cae for `RefCell<T>`:
*The name comes from the Cell pattern, which is used in Rust (and other languages) to provide controlled mutability.*
*A cell is a container that manages access to its inner value in a way that ensures safety according to a specific set of rules.*

We will explore writing tests for the following code:
```rs
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}
```

The following code fails to compile:
```rs
#[cfg(test)]
mod tests {
    use super::*;

    struct MockMessenger {
        sent_messages: Vec<String>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: vec![],
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.len(), 1);
    }
}
```
With the following error:
```txt
self.sent_messages.push(String::from(message));
   |             ^^^^^^^^^^^^^^^^^^ `self` is a `&` reference, so the data it refers to cannot be borrowed as mutable
   |
help: consider changing this to be a mutable reference in the `impl` method and the `trait` definition
   |
2  ~     fn send(&mut self, msg: &str);
3  | }
...
56 |     impl Messenger for MockMessenger {
57 ~         fn send(&mut self, message: &str) {
```

We can’t modify the MockMessenger to keep track of the messages, because the send method takes an immutable reference to self. 
We also can’t take the suggestion from the error text to use &mut self in both the impl method and the trait definition. 
We do not want to change the Messenger trait solely for the sake of testing. 
Instead, we need to find a way to make our test code work correctly with our existing design.

Instead we can use `RefCell<T>`:
```rs
#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        // --snip--

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
```

When creating immutable and mutable references, we use the & and &mut syntax, respectively.
With `RefCell<T>`, we use the `borrow` and `borrow_mut` methods, which are part of the safe API that belongs to `RefCell<T>`.
The `borrow` method returns the smart pointer type `Ref<T>`, and `borrow_mut` returns the smart pointer type `RefMut<T>`.
Both types implement `Deref`, so we can treat them like regular references.

The `RefCell<T>` keeps track of how many `Ref<T>` and `RefMut<T>` smart pointers are currently active.
Every time we call borrow, the `RefCell<T>` increases its count of how many immutable borrows are active.
When a `Ref<T>` value goes out of scope, the count of immutable borrows goes down by one.
Just like the compile-time borrowing rules, `RefCell<T>` lets us have many immutable borrows or one mutable borrow at any point in time.