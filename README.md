# Nom Snacks

More NOM parser combinators.

## Take All

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

## Find All

```rust
let input = "This is a {text} with some {special} {words}!";
let result = find_all::<_, _, _, nom::error::Error<_>>(
    "{",
    delimited(char('{'), alphanumeric1, char('}')),
)(input);
assert_eq!(Ok(("!", vec!["text", "special", "words"])), result);
```

## Recognize Separated

```rust
let input = "comma,separated,words other";
let result = recognize_separated0::<_, _, _, nom::error::Error<_>>(alphanumeric1, char(','))(input);
assert_eq!(Ok((" other", "comma,separated,words")), result);
```
