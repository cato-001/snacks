# nom snacks

[Github](https://github.com/cato-001/snacks)

[Crates.io](https://crates.io/crates/snacks)

[Documentation](https://docs.rs/snacks/latest/snacks/)

- parsers:
    - [combinators](#combinators)
    - [values](#values)

More useful parser-combinators for [nom](https://crates.io/crates/nom).

> **WARNING!** This project is still in development.
>
> If a parser looks useful, it may be more practical to copy it to your project.

To add this library to your project run:

```bash
cargo add snacks
```

# combinators

## find all

Run the parser at each found substring.

```rust
let input = "This is a {text} with some {special} {words}!";
let result = find_all::<_, _, _, nom::error::Error<_>>(
    "{",
    delimited(char('{'), alphanumeric1, char('}')),
)(input);
assert_eq!(Ok(("!", vec!["text", "special", "words"])), result);
```

> `find_all_into`
> This method can be used to push the items into a buffer, for saving allocations.

## recognize separated

Runs the item parser interlaced by the separator parser.

The main difference to the `separated_list0` parser from nom is,
that this parser returns the recognized string without allocating a list.

```rust
let input = "comma,separated,words other";
let result = recognize_separated0::<_, _, _, nom::error::Error<_>>(alphanumeric1, char(','))(input);
assert_eq!(Ok((" other", "comma,separated,words")), result);
```

## take all

Takes the items from the item parser, preceded by a prefix parser.

```rust
let input = "An example #sentence with #cool tags!";
let result = take_all::<_, _, nom::error::Error<_>>(
    is_not("#"),
    preceded(one_of("#"), is_not(" ")),
)(input);
assert_eq!(Ok((" tags!", vec!["sentence", "cool"])), result);
```

> `take_all_into`
> This method can be used to push the items into a buffer, for saving allocations.

# values


