
```rs
struct Point<T> {
  x: T,
  y: T,
}

impl<T> Point<T> {
  fn x(&self) -> &T {
    &self.x
  }
}

impl Point<f32> {
  fn distance_from_origin(&self) -> f32 {
    (self.x.powi(2) + self.y.powi(2)).sqrt()
  }
}

fn main() {
  let p = Point { x: 7, y: 7 };

  println!("p.x = {}", p.x());
}
```

Using generic types won’t make your program run any slower than it would with concrete types.
Rust accomplishes this by performing monomorphization of the code using generics at compile time.
Monomorphization is the process of turning generic code into specific code by filling in the concrete types that are used when compiled.