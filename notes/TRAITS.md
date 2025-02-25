A trait defines the functionality a particular type has and can share with other types. 
We can use traits to define shared behavior in an abstract way. 
We can use trait bounds to specify that a generic type can be any type that has certain behavior.

```rs
pub trait Summary {
  fn summarize(&self) -> String;
}
```

```rs
pub struct NewsArticle {
  pub headline: String,
  pub location: String,
  pub author: String,
  pub content: String,
}

impl Summary for NewsArticle {
  fn summarize(&self) -> String {
    format!("{}, by {} ({})", self.headline, self.author, self.location)
  }
}

pub struct Tweet {
  pub username: String,
  pub content: String,
  pub reply: bool,
  pub retweet: bool,
}

impl Summary for Tweet {
  fn summarize(&self) -> String {
    format!("{}: {}", self.username, self.content)
  }
}

```

## Default Implementation

```rs
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

impl Summary for NewsArticle {}
```

Default implementations can reference other methods defined in a trait...
```rs
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
```

## Traits as Parameters
```rs
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

Say we wanted notify to use display formatting as well as summarize on item: we specify in the notify definition that item must implement both Display and Summary. 
We can do so using the + syntax:
```rs
pub fn notify(item: &(impl Summary + Display)) {
  // snip ---
}
```

The + syntax is also valid with trait bounds on generic types:
```rs
pub fn notify<T: Summary + Display>(item: &T) {
  // snip ---
}
```

**where**
Using too many trait bounds has its downsides. 
Each generic has its own trait bounds, so functions with multiple generic type parameters can contain lots of trait bound information between the function’s name and its parameter list, making the function signature hard to read. 
For this reason, Rust has alternate syntax for specifying trait bounds inside a where clause after the function signature. 

So, instead of writing this:
```rs
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
```

we can use a where clause, like this:

```rs
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
```

**returning types that implement traits**

(However, you can only use impl Trait if you’re returning a single type.)

```rs
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}
```