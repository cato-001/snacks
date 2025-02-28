# NOM-Snacks

More NOM parser combinators.

## Take-All

```rust
let input = "An example #sentence with #cool tags!";
let result = take_all::<_, _, nom::error::Error<_>>(
    is_not("#"),
    preceded(one_of("#"), is_not(" ")),
)(input);
assert_eq!(Ok((" tags!", vec!["sentence", "cool"])), result);
```

Parse into buffer:
```rust
let mut buffer = Vec::new();
let input = "An example #sentence with #cool tags!";
let result = take_all_into::<_, _, nom::error::Error<_>>(
    is_not("#"),
    preceded(one_of("#"), is_not(" ")),
)(input, &mut buffer);
assert_eq!(Ok((" tags!", ())), result);
assert_eq!(vec!["sentence", "cool"], buffer);
```
