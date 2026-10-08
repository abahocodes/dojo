A string of `(` and `)` is **balanced** when every `(` is later closed by a
matching `)` and no `)` ever appears without an open `(` to close.

Given `n`, return every balanced string that uses exactly `n` opening and `n`
closing parentheses. Each string must appear once; the order of the list does
not matter.

## Example 1

```
n      = 3
output = ["((()))", "(()())", "(())()", "()(())", "()()()"]
```

## Example 2

```
n      = 1
output = ["()"]
```

## Constraints

- `1 <= n <= 8`
