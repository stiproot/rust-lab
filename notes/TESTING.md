# Test Organization
The Rust community thinks about tests in terms of two main categories: **unit tests** and **integration tests**. 
Unit tests are small and more focused, testing one module in isolation at a time, and can test private interfaces. 
Integration tests are entirely external to your library and use your code in the same way any other external code would, using only the public interface and potentially exercising multiple modules per test.

# Unit Tests
The #[cfg(test)] annotation on the tests module tells Rust to compile and run the test code only when you run cargo test, not when you run cargo build.
Unit tests go in the same files as the code, you’ll use #[cfg(test)] to specify that they shouldn’t be included in the compiled result.

*NOTE: cfg stands for configuration and tells Rust that the following item should only be included given a certain configuration option*

Rust’s privacy rules do allow you to test private functions.
```rs
pub fn add_two(a: usize) -> usize {
    internal_adder(a, 2)
}

fn internal_adder(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        let result = internal_adder(2, 2);
        assert_eq!(result, 4);
    }
}
```
This is becuase tests are just Rust code, and the tests module is just another module.

Items in child modules can use the items in their ancestor modules. 
In this test, we bring all of the tests module’s parent’s items into scope with use super::*, and then the test can call internal_adder. 

# Integration Tests
Integration tests go in a different directory, they don’t need the #[cfg(test)] annotation.

In Rust, integration tests are entirely external to your library. 
They use your library in the same way any other code would, which means they can only call functions that are part of your library’s public API.

## `tests` Dir
We create a tests directory at the top level of our project directory, next to src.
```txt
adder
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    └── integration_test.rs
```

*integration_tests.rs*
```rs
use adder::add_two;

#[test]
fn it_adds_two() {
    let result = add_two(2);
    assert_eq!(result, 4);
}
```

Each file in the tests directory is a separate crate, so we need to bring our library into each test crate’s scope.
For that reason we add use adder::add_two; at the top of the code, which we didn’t need in the unit tests.

# Binary Crates
If our project is a binary crate that only contains a src/main.rs file and doesn’t have a src/lib.rs file, we can’t create integration tests in the tests directory and bring functions defined in the src/main.rs file into scope with a use statement. 
Only library crates expose functions that other crates can use; binary crates are meant to be run on their own.
