Given a non-negative integer `x`, return the integer part of its square root,
that is, the largest integer `r` with `r * r <= x`.

You may **not** use a built-in square-root or exponentiation function or
operator (such as `sqrt`, `pow`, `**` or `isqrt`).

## Example 1

```
x      = 27
output = 5    # 5 * 5 = 25 <= 27 < 36 = 6 * 6
```

## Example 2

```
x      = 144
output = 12
```

## Constraints

- `0 <= x <= 2^52`
- The answer is therefore at most `2^26`, so `r * r` never exceeds `2^52`
  if you keep `r` in that range.
