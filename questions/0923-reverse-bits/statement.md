You are given `n`, an unsigned 32-bit integer (a value from `0` to
`2^32 - 1`). Write `n` as exactly 32 binary digits, with leading zeros, then
reverse the order of those 32 digits: bit `0` (the lowest) moves to bit `31`,
bit `1` to bit `30`, and so on. Return the value the reversed digits represent,
again as an unsigned 32-bit integer.

`n` and the result are passed as 64-bit integers (`long`) so that every
language sees the same non-negative value, including those above `2^31 - 1`.

## Example 1

```
n      = 43261596
output = 964176192
# n      = 00000010100101000001111010011100
# output = 00111001011110000010100101000000
```

## Example 2

```
n      = 4294967293
output = 3221225471
# n      = 11111111111111111111111111111101
# output = 10111111111111111111111111111111
```

## Constraints

- `0 <= n <= 2^32 - 1`

**Follow-up:** if this were called many times, how could you make each call
faster?
