# Approach: shift bits out of `n` and into the result

Treat `n` as a queue of 32 bits read from the low end, and the result as a
stack that grows from its low end. Each step takes the lowest bit of `n` and
pushes it onto the result:

```python
def reverse_bits(n):
    result = 0
    for _ in range(32):
        result = (result << 1) | (n & 1)
        n >>= 1
    return result
```

The first bit read (bit `0` of `n`) is shifted left 31 more times, so it ends
in bit 31; the last bit read (bit 31) stays in bit 0. The loop must run all 32
times even after `n` becomes `0`, because the remaining zeros still push the
earlier bits upward.

**Follow-up (many calls):** split the 32 bits into four bytes, reverse each
byte with a precomputed 256-entry table, and place the reversed bytes in the
opposite order. Another constant-time trick swaps halves, then quarters, and
so on with masks:

```
n = (n >> 16) | (n << 16)                                 (keep 32 bits)
n = ((n & 0xff00ff00) >> 8) | ((n & 0x00ff00ff) << 8)
n = ((n & 0xf0f0f0f0) >> 4) | ((n & 0x0f0f0f0f) << 4)
n = ((n & 0xcccccccc) >> 2) | ((n & 0x33333333) << 2)
n = ((n & 0xaaaaaaaa) >> 1) | ((n & 0x55555555) << 1)
```

## Complexity

- Time: O(32) = O(1).
- Space: O(1).

## Pitfalls

- Stopping when `n` reaches `0` skips the shifts that place the bits; `1`
  would come out as `1` instead of `2147483648`.
- Reversing `bin(n)` without padding to 32 digits loses the leading zeros.
- In JavaScript, `result << 1` is a signed 32-bit operation: once bit 31 is
  set, the value turns negative. Finish with `>>> 0` to read it as unsigned.
- In Java or C++ with 32-bit `int`, the result may look negative. Here the
  value travels as a `long`, so a plain 64-bit shift-and-or works; just never
  let it grow past 32 bits.
