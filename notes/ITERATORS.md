
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

`filter`:
```rs
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}
```
We use filter with a closure that captures the shoe_size variable from its environment to iterate over a collection of Shoe struct instances. It will return only shoes that are the specified size.