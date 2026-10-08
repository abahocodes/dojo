Call a positive integer **magical** if it is a multiple of `a`, a multiple of
`b`, or both. List the magical numbers in increasing order, each appearing
once. Return the `n`-th one (counting from 1), reduced modulo `10^9 + 7`.

## Example 1

```
n = 5, a = 4, b = 6
output = 16   # magical numbers: 4, 6, 8, 12, 16, ...
```

## Example 2

```
n = 4, a = 3, b = 3
output = 12   # 3, 6, 9, 12
```

## Constraints

- `1 <= n <= 10^9`
- `2 <= a, b <= 4 * 10^4`

The true (unreduced) answer can be as large as `4 * 10^13`.
