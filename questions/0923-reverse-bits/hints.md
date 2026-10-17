# Hints

## Hint 1
Leading zeros count: `1` reverses to `2147483648` (a `1` in bit 31), not `1`.
You always handle exactly 32 positions.

## Hint 2
Build the answer one bit at a time. Read the lowest bit of `n` with `n & 1`,
then shift `n` right; shift the answer left to make room before adding the bit.

## Hint 3
Loop 32 times: `result = (result << 1) | (n & 1)`, then `n >>= 1`. After 32
steps the bit that started at position `0` has been shifted up to position 31.
