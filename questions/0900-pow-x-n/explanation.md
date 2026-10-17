# Approach: binary exponentiation (fast power)

Write `n` in binary. For `n = 13 = 1101₂`, `x¹³ = x⁸ · x⁴ · x¹`. The powers
`x, x², x⁴, x⁸, ...` come from repeatedly squaring `x`, and the bits of `n` say
which of them to multiply into the answer. That's one squaring and at most one
multiplication per bit: about 31 iterations for any 32-bit exponent.

A negative exponent is handled up front: `x⁻ᵏ = (1/x)ᵏ`.

```python
def my_pow(x, n):
    if n < 0:
        x = 1 / x
        n = -n
    result = 1.0
    while n > 0:
        if n & 1:
            result *= x
        x *= x
        n >>= 1
    return result
```

The recursive form says the same thing: `pow(x, n) = half * half` (times `x`
if `n` is odd) where `half = pow(x, n // 2)`. Compute `half` once; calling
`pow(x, n // 2)` twice makes it O(n) again.

## Complexity

- Time: O(log |n|) multiplications.
- Space: O(1) iteratively (O(log |n|) stack for the recursive version).

## Pitfalls

- **`n = -2^31`.** In Java and C++, `-n` overflows a 32-bit `int` and stays
  negative. Copy `n` into a `long`/`long long` before negating. (In
  JavaScript, avoid `>>` on the negated value: `2^31` is outside the 32-bit
  range bitwise operators use; divide by 2 instead.)
- A loop of `|n|` multiplications times out on large exponents, even for
  `x = 1` or `x = -1` unless you special-case them.
- The sign of a negative base depends on the parity of `n`:
  `(-1)^(2^31 - 1) = -1` but `(-1)^(-2^31) = 1`. Binary exponentiation gets
  this right automatically.
- Integer `0` handling: `x⁰ = 1` for every allowed `x`, and the loop above
  returns `1.0` without entering it when `n = 0`.
