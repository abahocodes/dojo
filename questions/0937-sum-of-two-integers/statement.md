Given two signed 32-bit integers `a` and `b`, return their sum **without
using the `+` or `-` operators** (and without library calls that add for you,
such as `sum`). Bitwise operators (`&`, `|`, `^`, `~`, shifts) are allowed.

The tests cannot see which operators your code uses, so this rule is on your
honour. It is the whole point of the question.

The inputs are chosen so that the true sum always fits in a signed 32-bit
integer.

## Example 1

```
a      = 4
b      = 9
output = 13
```

## Example 2

```
a      = -6
b      = 2
output = -4
```

## Constraints

- `-2^30 <= a, b <= 2^30 - 1`
