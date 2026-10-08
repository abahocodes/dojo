Given a positive integer `num`, return `true` if `num` is a **perfect
square**, meaning `num = r * r` for some integer `r`, and `false` otherwise.

You may **not** use a built-in square-root or exponentiation function or
operator (such as `sqrt`, `pow`, `**` or `isqrt`).

## Example 1

```
num    = 49
output = true    # 7 * 7
```

## Example 2

```
num    = 50
output = false   # 7 * 7 = 49 < 50 < 64 = 8 * 8
```

## Constraints

- `1 <= num <= 2^52`
- Any root `r` is therefore at most `2^26`.
