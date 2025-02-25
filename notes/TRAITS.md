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