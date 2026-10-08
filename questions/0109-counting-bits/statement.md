For every whole number `i` from `0` up to and including `n`, count how many
`1` bits appear in the binary form of `i`. Return the counts as a list of
length `n + 1`, where position `i` holds the count for `i`.

## Example 1

```
n      = 4
output = [0, 1, 1, 2, 1]
# 0 = 0, 1 = 1, 2 = 10, 3 = 11, 4 = 100
```

## Example 2

```
n      = 7
output = [0, 1, 1, 2, 1, 2, 2, 3]
```

## Constraints

- `0 <= n <= 10^5`

**Follow-up:** counting the bits of each number separately is O(n log n). Can
you produce the whole list in O(n), reusing earlier answers?
