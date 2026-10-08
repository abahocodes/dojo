A string of `(` and `)` is **balanced** when every `(` has a matching `)`
after it and every `)` closes an earlier `(`. Any non-empty balanced string can
be cut, in exactly one way, into a sequence of **blocks**: balanced pieces that
cannot themselves be cut into two smaller non-empty balanced pieces. For
example, `"(())()"` is made of the blocks `"(())"` and `"()"`.

Given a balanced string `s`, strip the first and last character of every block
and return what is left, keeping the remaining characters in their original
order.

## Example 1

```
s      = "(()())(())"
output = "()()()"   # blocks "(()())" and "(())" become "()()" and "()"
```

## Example 2

```
s      = "()()((()))"
output = "(())"     # "()" and "()" vanish, "((()))" becomes "(())"
```

## Constraints

- `1 <= len(s) <= 10^5`
- `s` contains only `(` and `)` and is balanced (so its length is even).
