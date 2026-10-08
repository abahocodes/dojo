A string `s` is made only of the characters `'('` and `')'`.

A string is **balanced** when it is empty, or it is `"(" + A + ")"` for a
balanced `A`, or it is the concatenation `A + B` of two balanced strings. For
example `"()"`, `"(())"` and `"()(())"` are balanced, while `")("` and `"(()"`
are not.

Return the length of the longest contiguous piece of `s` that is balanced.
The empty piece always qualifies, so the answer is `0` when nothing longer is
balanced.

## Example 1

```
s      = ")()(())(("
output = 6    # "()(())" starting at index 1
```

## Example 2

```
s      = "(()"
output = 2    # "()"
```

## Constraints

- `0 <= len(s) <= 3 * 10^4`
- every character of `s` is `'('` or `')'`
