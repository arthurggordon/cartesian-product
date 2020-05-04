# Cartesian Product Iterator

This is an iterator that produces a N-dimensional cartesian product.
 
For example, the cartesian product of:
``` 
[ [1, 2], [3, 4], [5, 6, 7] ]
```
is 
```
[
    [1, 3, 5], [2, 3, 5], [1, 4, 5], [2, 4, 5],
    [1, 3, 6], [2, 3, 6], [1, 4, 6], [2, 4, 6],
    [1, 3, 7], [2, 3, 7], [1, 4, 7], [2, 4, 7]
]

```

## Run
To execute the code:
```
cargo run
```
## Test 
To run the unit tests
```
cargo test
```