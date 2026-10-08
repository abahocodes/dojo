Given an integer `n`, return all integers from `1` to `n` ordered as their
decimal strings would appear in a dictionary: compare digit by digit, and a
string that is a prefix of another comes first.

Aim for O(n) time and O(1) extra space besides the output.

## Example 1

```
n      = 13
output = [1, 10, 11, 12, 13, 2, 3, 4, 5, 6, 7, 8, 9]
```

## Example 2

```
n      = 2
output = [1, 2]
```

## Constraints

- `1 <= n <= 5 * 10^4`
