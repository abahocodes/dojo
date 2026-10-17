A non-negative integer is stored as an array of its decimal digits, most
significant digit first: `[4, 3, 2, 1]` stands for `4321`. Add one to the
number and return the result in the same form.

The number has no leading zeros, except for the number zero itself, which is
stored as `[0]`. The input can have up to 100 digits, so it does not fit in a
built-in integer type.

## Example 1

```
digits = [4, 3, 2, 1]
output = [4, 3, 2, 2]
```

## Example 2

```
digits = [9, 9]
output = [1, 0, 0]
```

`99 + 1 = 100`: the carry ripples through every digit and the result is one
digit longer.

## Example 3

```
digits = [0]
output = [1]
```

## Constraints

- `1 <= len(digits) <= 100`
- `0 <= digits[i] <= 9`
- `digits` has no leading zero unless it is exactly `[0]`.
