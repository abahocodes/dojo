You are given a valid arithmetic expression `s` made of digits, the operators
`+`, `-`, `*`, `/`, and parentheses. Its parentheses are properly matched.

The **nesting depth** of a position is the number of parenthesis pairs that
enclose it. Return the largest nesting depth anywhere in `s`, or `0` if `s`
contains no parentheses.

## Example 1

```
s      = "(4+(2*3))-((5)/1)"
output = 2
# "2*3" sits inside two pairs, and so does "5".
```

## Example 2

```
s      = "8*9"
output = 0
```

## Constraints

- `1 <= len(s) <= 10^4`
- `s` consists of digits and the characters `+`, `-`, `*`, `/`, `(` and `)`.
- `s` is a valid expression, so its parentheses are balanced and properly
  nested.
