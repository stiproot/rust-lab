The function signature tells Rust that for some lifetime 'a, the function takes two parameters, both of which are string slices that live at least as long as lifetime 'a. 

The function signature also tells Rust that the string slice returned from the function will live at least as long as lifetime 'a.

In practice, it means that the lifetime of the reference returned by the longest function is the same as the smaller of the lifetimes of the values referred to by the function arguments.

We’re specifying that the borrow checker should reject any values that don’t adhere to these constraints.

When we pass concrete references to longest, the concrete lifetime that is substituted for 'a is the part of the scope of x that overlaps with the scope of y. 
In other words, the generic lifetime 'a will get the concrete lifetime that is equal to the smaller of the lifetimes of x and y. 
Because we’ve annotated the returned reference with the same lifetime parameter 'a, the returned reference will also be valid for the length of the smaller of the lifetimes of x and y.

```rs
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

# Lifetime annotation in struct definition:
```rs
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
```

# Lifetime Elision
```rs
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```
The reason this function compiles without lifetime annotations is historical: in early versions (pre-1.0) of Rust, this code wouldn’t have compiled because every reference needed an explicit lifetime. 
At that time, the function signature would have been written like this:
```rs
fn first_word<'a>(s: &'a str) -> &'a str { ... }
```
Some situations are predictable and follow a few deterministic patterns. 
The developers programmed these patterns into the compiler’s code so the borrow checker could infer the lifetimes in these situations and wouldn’t need explicit annotations.

The patterns programmed into Rust’s analysis of references are called the lifetime elision rules.
These aren’t rules for programmers to follow; they’re a set of particular cases that the compiler will consider, and if your code fits these cases, you don’t need to write the lifetimes explicitly.

The elision rules don’t provide full inference. 
If there is still ambiguity as to what lifetimes the references have after Rust applies the rules, the compiler won’t guess what the lifetime of the remaining references should be. 
Instead of guessing, the compiler will give you an error that you can resolve by adding the lifetime annotations.

The compiler uses three rules to figure out the lifetimes of the references when there aren’t explicit annotations. 
The first rule applies to input lifetimes, and the second and third rules apply to output lifetimes. 
If the compiler gets to the end of the three rules and there are still references for which it can’t figure out lifetimes, the compiler will stop with an error. 
These rules apply to fn definitions as well as impl blocks.

The first rule is that the compiler assigns a lifetime parameter to each parameter that’s a reference. 
In other words, a function with one parameter gets one lifetime parameter: fn foo<'a>(x: &'a i32); a function with two parameters gets two separate lifetime parameters: fn foo<'a, 'b>(x: &'a i32, y: &'b i32); and so on.

The second rule is that, if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters: fn foo<'a>(x: &'a i32) -> &'a i32.

The third rule is that, if there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, the lifetime of self is assigned to all output lifetime parameters. 
This third rule makes methods much nicer to read and write because fewer symbols are necessary.


**Syntax of specifying generic type parameters, trait bounds, and lifetimes all in one function!**

```rs
fn longest_with_an_announcement<'a, T>(
  x: &'a str, 
  y: &'a, 
  ann: T
) -> &'a str
where 
  T: Display, 
{
  println!("Announcement: {ann}");
  if x.len() > y.len() {
    x
  }
  y
}
```