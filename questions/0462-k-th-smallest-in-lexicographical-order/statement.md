Write the integers `1, 2, ..., n` as decimal strings (no leading zeros) and
sort those strings in **dictionary order**: compare character by character,
and a string that is a proper prefix of another comes first.

Return the integer that sits at position `k` (1-based) in that order.

## Example 1

```
n      = 13
k      = 4
output = 12
```

The order is `1, 10, 11, 12, 13, 2, 3, 4, 5, 6, 7, 8, 9`; the fourth entry is
`12`.

## Example 2

```
n      = 120
k      = 30
output = 17
```

The order begins `1, 10, 100, ..., 109, 11, 110, ..., 119, 12, 120, 13, 14, 15,
16, 17, ...`: 25 numbers up to and including `120`, then `13` through `17`.

## Constraints

- `1 <= k <= n <= 10^9`
