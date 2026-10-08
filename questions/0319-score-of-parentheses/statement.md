A balanced string of parentheses is given a numeric score by these rules:

- the pair `"()"` scores `1`;
- placing two balanced strings `A` and `B` side by side (`AB`) scores
  `score(A) + score(B)`;
- wrapping a balanced string `A` in a pair (`(A)`) scores `2 * score(A)`.

Given a balanced string `s`, return its score.

## Example 1

```
s      = "(()())"
output = 4     # inside: "()" + "()" = 2, wrapped: 2 * 2
```

## Example 2

```
s      = "((()))()"
output = 5     # "((()))" = 4 and "()" = 1
```

## Constraints

- `2 <= len(s) <= 50`
- `s` consists only of `(` and `)` and is balanced.
