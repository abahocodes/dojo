You are given a string `s` holding a valid arithmetic expression. Evaluate it
and return the result, without using any built-in expression evaluator
(`eval` and the like).

The expression is made of:

- non-negative integer literals (decimal digits, no leading zeros except the
  number `0` itself);
- the binary operators `"+"` and `"-"`, applied left to right;
- parentheses `"("` and `")"` for grouping;
- spaces, which may appear anywhere between tokens and carry no meaning.

A `"-"` can also be **unary** (negation), but only at the very start of the
expression or right after a `"("` — for example `"-7"` or `"3 - (-(2 + 1))"`.
A `"+"` is never unary, and two operators never appear next to each other.

## Example 1

```
s = "7 - (2 + 3) + 4"
output = 6
```

## Example 2

```
s = "-(10 - (4+ 9))"
output = 3
```

`4 + 9 = 13`, `10 - 13 = -3`, and negating gives `3`.

## Constraints

- `1 <= len(s) <= 3 * 10^5`
- `s` contains only digits, `"+"`, `"-"`, `"("`, `")"` and spaces, and is a
  valid expression as described above.
- Every integer literal, every intermediate result and the final answer fit in
  a 32-bit signed integer.
