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
