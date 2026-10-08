You are keeping score for an unusual game. The score record starts empty, and
you are given a list of `operations` to apply in order. Each operation is one
of:

- an integer written as a string, such as `"7"` or `"-12"`: add that score to
  the record;
- `"+"`: add a new score equal to the sum of the last two scores in the record;
- `"D"`: add a new score equal to twice the last score in the record;
- `"C"`: remove the last score from the record. It no longer counts.

Return the sum of all scores left in the record after every operation has
been applied (`0` if the record ends up empty).

## Example 1

```
operations = ["4", "-2", "D", "C", "+"]
output     = 4
# record: [4] -> [4, -2] -> [4, -2, -4] -> [4, -2] -> [4, -2, 2]
```

## Example 2

```
operations = ["10", "3", "+", "D", "C", "C", "7"]
output     = 20
# record: [10] -> [10, 3] -> [10, 3, 13] -> [10, 3, 13, 26]
#         -> [10, 3, 13] -> [10, 3] -> [10, 3, 7]
```

## Constraints

- `1 <= len(operations) <= 1000`
- Every operation is `"+"`, `"D"`, `"C"`, or an integer in
  `[-3 * 10^4, 3 * 10^4]`.
- Every operation is valid: `"+"` only appears when the record holds at least
  two scores, and `"D"` and `"C"` only when it holds at least one.
- Every score in the record and the final sum fit in a 32-bit signed integer.
