
```yml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```
The opt-level setting controls the number of optimizations Rust will apply to your code, with a range of 0 to 3.

*Documentation comments*
```rs
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

*Documentation that describes the purpose of the my_crate crate that contains the add_one function, we add documentation comments that start with //! to the beginning of the src/lib.rs file*
```rs
//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.

/// Adds one to the number given.
// --snip--
```

*Package metadata*
```yml
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2021"
description = "A fun game where you guess what number the computer has chosen."
license = "MIT OR Apache-2.0"

[dependencies]
```

*Yanking*
Yanking a version prevents new projects from depending on that version while allowing all existing projects that depend on it to continue. 
Essentially, a yank means that all projects with a Cargo.lock will not break, and any future Cargo.lock files generated will not use the yanked version.

The `cargo install` command allows you to install and use binary crates locally.
Note that you can only install packages that have binary targets. 
A binary target is the runnable program that is created if the crate has a src/main.rs file or another file specified as a binary, as opposed to a library target that isn’t runnable on its own but is suitable for including within other programs. 