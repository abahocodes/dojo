# Approach: clear the lowest set bit until nothing is left

Subtracting `1` from `n` turns its lowest `1` bit into `0` and every `0` below
it into `1`; the bits above stay the same. ANDing with the original therefore
removes exactly that lowest `1` bit:

```
n         = 1011 0100
n - 1     = 1011 0011
n & (n-1) = 1011 0000
```

Each iteration removes one set bit, so the loop runs once per `1` bit
(Brian Kernighan's trick).

```python
def hamming_weight(n):
    count = 0
    while n:
        n &= n - 1
        count += 1
    return count
```

**Simpler alternative:** test the low bit and shift, 32 times at most:
`count += n & 1; n >>= 1`. Both are O(32) in the worst case; Kernighan's loop
is faster on sparse inputs.

**Library calls:** `bin(n).count("1")` / `n.bit_count()` in Python,
`Long.bitCount` in Java, `__builtin_popcountll` in C++, `bits.OnesCount` in Go.
Fine in practice, but interviewers usually want the loop.

## Complexity

- Time: O(k) where `k` is the number of `1` bits, at most 32.
- Space: O(1).

## Pitfalls

- In JavaScript, bitwise operators work on signed 32-bit values: for
  `n >= 2^31`, `n & (n - 1)` comes out negative and `while (n)` still ends,
  but `n >> 1` would keep the sign bit and loop forever. Use `>>>` (unsigned
  shift) or, as here, the `n & (n - 1)` loop with `!== 0`.
- In Java or C++, if you take the value as a 32-bit `int`, values above
  `2^31 - 1` are negative; shifting them with `>>` sign-extends forever. Use
  `>>>` in Java or an unsigned type in C++. With the `long` input here this
  does not come up.
