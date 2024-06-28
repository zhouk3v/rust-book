Patterns that will match for any possible value passed are irrefutable

E.g. The pattern `x` in the statement `let x = 5`

Patterns that can fail to match for some possible value are refutable.

E.g:

- In `if let Some(x) = a_value`, `Some(x)` is refutable: if `a_value` is `None`, the `Some(x)` pattern will not match
- In `if let &[x, ..] = a_slice`, then `&[x, ..]` is refutable. If the value in the `a_slice` variable has zero elements, the `&[x, ..]` pattern will not match.

Function parameters, `let` statements and `for` loops only accept irrefutable patterns, beacuse the program cannot do anything when values don't match

`if let` and `while let` expressions accept refutable and irrefutable patterns, but the compiler will warn against irrefutable patterns because `if let` and `while let` are designed to handle possible failure.

You should be familiar with the concept of refutability to understand it when seeing it in an error message.

E.g. This code will not compile:

```Rust
let Some(x) = some_option_value;
```

If `some_option_value` was a `None` value, then it would fail to match the pattern `Some(x)`, so the pattern is refutable. However, the `let` statement only accepts irrefutable patterns because there is nothing valid that the code can do with a `None` value. This will be caught at compile time.

If we have a refutable pattern where an irrefutable pattern is needed, it can be fixed by changing the code that uses the pattern (such as using `if let` instead of `let`). If the pattern doesn't match, the code in the scope of the `if let` will be skipped:

```Rust
if let Some(x) = some_option_value {
  println!("{}", x);
}
```

If we give `if let` a pattern that always matches, the compiler will give a warning

```Rust
if let x = 5 {
  println!("{}",x);
}
```

Rust will say that it doesn't make sense to use `if let` with an irrefutable pattern

For this reason, match arms must use refutable patterns, except for the last arm, which should use a irrefutable pattern.

Rust will allow a match with only one arm with an irrefutable pattern, but it is not particularly useful and could be replaced with a `let` statement instead
