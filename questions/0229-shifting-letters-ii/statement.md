You are given a lowercase string `s` and a list of operations `shifts`, where
`shifts[i] = [start, end, direction]` acts on every character of `s` at
indices `start` through `end`, inclusive (0-based):

- if `direction` is `1`, each of those letters moves **forward** one place in
  the alphabet (`'a'` becomes `'b'`, ..., and `'z'` wraps around to `'a'`);
- if `direction` is `0`, each moves **backward** one place (`'b'` becomes
  `'a'`, ..., and `'a'` wraps around to `'z'`).

Apply every operation and return the resulting string. The order in which the
operations are applied does not affect the result.

## Example 1

```
s      = "cat"
shifts = [[0, 1, 1], [1, 2, 0]]
output = "das"   # c +1 -> d, a +1 -1 -> a, t -1 -> s
```

## Example 2

```
s      = "az"
shifts = [[0, 1, 0], [1, 1, 1], [1, 1, 1]]
output = "za"    # a -1 wraps to z, z -1 +1 +1 wraps to a
```

## Constraints

- `1 <= len(s) <= 5 * 10^4`
- `1 <= len(shifts) <= 5 * 10^4`
- `shifts[i].length == 3`
- `0 <= start <= end < len(s)`
- `direction` is `0` or `1`
- `s` consists of lowercase English letters.
