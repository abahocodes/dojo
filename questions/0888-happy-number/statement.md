Take a positive integer and replace it by the sum of the squares of its decimal
digits. Repeat. If this process eventually reaches `1` (where it stays, since
`1² = 1`), the starting number is called **happy**. Otherwise the process loops
forever through a cycle that never contains `1`, and the number is unhappy.

Given `n`, return `true` if it is happy and `false` otherwise.

## Example 1

```
n      = 19
output = true
```

`1² + 9² = 82`, `8² + 2² = 68`, `6² + 8² = 100`, `1² + 0² + 0² = 1`.

## Example 2

```
n      = 4
output = false
```

`4 → 16 → 37 → 58 → 89 → 145 → 42 → 20 → 4`, and the cycle repeats without
ever hitting `1`.

## Example 3

```
n      = 1
output = true
```

## Constraints

- `1 <= n <= 2^31 - 1`
