
Methods that call next are called `consuming adapters`.

One example is the sum method, which takes ownership of the iterator and iterates through the items by repeatedly calling next, thus consuming the iterator. 
```rs
#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}
```

We aren’t allowed to use v1_iter after the call to sum because sum takes ownership of the iterator we call it on.

`Iterator adapters` are methods defined on the Iterator trait that don’t consume the iterator.
Instead, they produce different iterators by changing some aspect of the original iterator.
`Iterator adapters` are lazy, and so you have to call one of the `consuming adapter` methods to get results from calls to `iterator adapters`.

`map`, which takes a closure to call on each item as the items are iterated through. 
The map method returns a new iterator that produces the modified items. 
The closure here creates a new iterator in which each item from the vector will be incremented by 1:
```rs
let v1: Vec<i32> = vec![1, 2, 3];

v1.iter().map(|x| x + 1);
```

The `collect` method consumes the iterator and collects the resulting values into a collection data type.
```rs
let v1: Vec<i32> = vec![1, 2, 3];

let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

assert_eq!(v2, vec![2, 3, 4]);
```