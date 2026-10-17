Compute `x` raised to the power `n`, where `x` is a floating-point number and
`n` is a (possibly negative) 32-bit integer. Don't call the language's
built-in power function (`**`, `Math.pow`, `pow`, `math.Pow`, ...); build it
from multiplication.

For negative `n`, `xⁿ` means `1 / x^(-n)`. Any `x` to the power `0` is `1`.

`n` can be as large as `2^31 - 1` or as small as `-2^31`, so multiplying `x` in
one step at a time is far too slow.

Answers are compared with a tolerance of `1e-6` (relative to the expected
value once it's larger than 1).

## Example 1

```
x      = 2.0
n      = 10
output = 1024.0
```

## Example 2

```
x      = 2.5
n      = 3
output = 15.625
```

## Example 3

```
x      = 2.0
n      = -3
output = 0.125
```

`2⁻³ = 1 / 2³ = 1 / 8`.

## Constraints

- `-100.0 < x < 100.0`
- `-2^31 <= n <= 2^31 - 1`
- `x` is not `0` when `n <= 0`.
- The exact value of `xⁿ` lies between `-10^4` and `10^4`.
