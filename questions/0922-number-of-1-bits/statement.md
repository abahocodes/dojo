You are given `n`, an unsigned 32-bit integer (a value from `0` to
`2^32 - 1`). Return how many of its 32 binary digits are `1`. This count is
also called the number's *population count* or *Hamming weight*.

`n` is passed as a 64-bit integer (`long`) so that every language sees the
same non-negative value, including those above `2^31 - 1`.

## Example 1

```
n      = 11
output = 3
# 11 = 1011 in binary
```

## Example 2

```
n      = 128
output = 1
# 128 = 10000000
```

## Example 3

```
n      = 4294967293
output = 31
# 11111111111111111111111111111101
```

## Constraints

- `0 <= n <= 2^32 - 1`

**Follow-up:** can you make the loop run once per `1` bit instead of once per
bit position?
