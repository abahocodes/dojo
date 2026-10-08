Given four positive integers `n`, `a`, `b` and `c`, call a positive integer
**ugly** if it is divisible by at least one of `a`, `b` or `c`.

List the ugly numbers in increasing order (each number appears once, even if
it is divisible by several of `a`, `b`, `c`). Return the `n`-th number of
that list, counting from 1.

## Example 1

```
n      = 4
a      = 2
b      = 3
c      = 4
output = 6     # ugly numbers: 2, 3, 4, 6, 8, 9, ...
```

## Example 2

```
n      = 5
a      = 3
b      = 3
c      = 7
output = 12    # ugly numbers: 3, 6, 7, 9, 12, ...
```

## Constraints

- `1 <= n, a, b, c <= 10^9`
- The answer is guaranteed to be at most `2 * 10^9`.
