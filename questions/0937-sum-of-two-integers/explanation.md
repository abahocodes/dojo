# Approach: XOR for the sum digits, AND-and-shift for the carries

Adding one column of binary digits gives a sum bit and a carry bit:

| x | y | sum `x ^ y` | carry `x & y` |
|---|---|-------------|---------------|
| 0 | 0 | 0 | 0 |
| 0 | 1 | 1 | 0 |
| 1 | 0 | 1 | 0 |
| 1 | 1 | 0 | 1 |

So for whole numbers, `a ^ b` is the sum with every carry ignored, and
`(a & b) << 1` is exactly the carries, moved to the column they belong in.
`a + b` equals `(a ^ b) + ((a & b) << 1)`, which is another addition, so
repeat it until the carry part is `0`:

```
while b != 0:
    carry = (a & b) << 1
    a = a ^ b
    b = carry
return a
```

In two's-complement 32-bit arithmetic this works for negative numbers as well;
every carry moves at least one position left each round, so after at most 32
rounds it falls off the top and `b` becomes `0`.

```java
class Solution {
    public int getSum(int a, int b) {
        while (b != 0) {
            int carry = (a & b) << 1;
            a = a ^ b;
            b = carry;
        }
        return a;
    }
}
```

## Python: simulate 32 bits with a mask

Python integers have no fixed width, and a negative number behaves as if it
had infinitely many leading `1` bits. With `a = -1, b = 1` the carry would
march left forever. The fix is to keep every intermediate value to its low 32
bits with `MASK = 0xFFFFFFFF`, and at the end turn the 32-bit pattern back
into a signed number: if bit 31 is set, the value is negative, and
`~(a ^ MASK)` flips the low 32 bits twice while making all higher bits `1`.

```python
def get_sum(a, b):
    MASK = 0xFFFFFFFF
    MAX = 0x7FFFFFFF
    a &= MASK
    b &= MASK
    while b:
        carry = ((a & b) << 1) & MASK
        a = a ^ b
        b = carry
    return a if a <= MAX else ~(a ^ MASK)
```

Example `a = -6, b = 2`: masked, `a = 0xFFFFFFFA`, `b = 0x2`. Round 1:
carry `= (0xFFFFFFFA & 0x2) << 1 = 0x4`, `a = 0xFFFFFFF8`. Round 2: carry
`= 0`, `a = 0xFFFFFFFC`, which has bit 31 set, so the answer is
`~(0xFFFFFFFC ^ 0xFFFFFFFF) = ~3 = -4`.

In other languages:

- **C++:** shifting a negative `int` left is undefined behaviour before C++20,
  so do the work in `unsigned int` and cast back.
- **JavaScript / TypeScript:** `&`, `^` and `<<` already convert to signed
  32-bit integers, so the plain loop works.
- **Go:** `int` is 64 bits, which also works since the sum fits, but the
  solution here uses `int32` to stay faithful to the 32-bit model.

## Complexity

- Time: O(32) = O(1): each round moves the lowest carry at least one bit left.
- Space: O(1).

## Pitfalls

- Python without the mask loops forever on mixed-sign inputs such as `-1, 1`.
- Forgetting to convert the masked result back: `0xFFFFFFFC` must be returned
  as `-4`, not `4294967292`.
- Updating `a` before computing the carry from the old `a`.
