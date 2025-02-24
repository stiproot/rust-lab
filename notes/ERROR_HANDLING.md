
Rust groups errors into two major categories: recoverable and unrecoverable errors.

Rust doesn’t have exceptions. Instead, it has the type Result<T, E> for recoverable errors and the panic! macro that stops execution when the program encounters an unrecoverable error.

# Unrecoverable Errors
There are two ways to cause a panic in practice: by taking an action that causes our code to panic (such as accessing an array past the end) or by explicitly calling the panic! macro. 
In both cases, we cause a panic in our program. 
By default, these panics will print a failure message, unwind, clean up the stack, and quit. 
Via an environment variable, you can also have Rust display the call stack when a panic occurs to make it easier to track down the source of the panic.

By default, when a panic occurs the program starts unwinding, which means Rust walks back up the stack and cleans up the data from each function it encounters. 
However, walking back and cleaning up is a lot of work. 
Rust, therefore, allows you to choose the alternative of immediately aborting, which ends the program without cleaning up.

Memory that the program was using will then need to be cleaned up by the operating system. 
If in your project you need to make the resultant binary as small as possible, you can switch from unwinding to aborting upon a panic by adding panic = 'abort' to the appropriate [profile] sections in your Cargo.toml file. 
For example, if you want to abort on panic in release mode, add this:

```rs
[profile.release]
panic = 'abort'
```


```rs
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            }
        },
    };
}
```

Unwrap:

```rs
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt").unwrap();
}
```

```rs
fn main() {
   let greeting_file = File::open("hello.txt")
    .expect("hello.txt should be added to the solution.");
}
```

The following...
```rs
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
```

Can be simplified as ...

```rs
use std::fs:File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
  let username_file = File::open("hello.txt")?;
  let mut username = String::new();
  username_file.read_to_string(&mut username;)
  Ok(username)
}
```

And even further simplified as...

```rs
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}
```

And even further simplified (:D) as...
```rs
use std::fs;

fn read_username_from_file() -> Result<String, io::Error> {
  fs::read_to_string("hello.txt")
}
```

If the value of the Result is an Ok, the value inside the Ok will get returned from this expression, and the program will continue.
If the value is an Err, the Err will be returned from the whole function as if we had used the return keyword so the error value gets propagated to the calling code.

The ? operator can only be used in functions whose return type is compatible with the value the ? is used on.

As with using ? on Result, you can only use ? on Option in a function that returns an Option. 
The behavior of the ? operator when called on an Option<T> is similar to its behavior when called on a Result<T, E>: if the value is None, the None will be returned early from the function at that point. 
If the value is Some, the value inside the Some is the resultant value of the expression, and the function continues.

```rs
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
```
