# Approach: carry from the right

Adding one only changes a suffix of trailing `9`s plus the digit just before
it. Walk from the end:

- a `9` becomes `0` and the carry continues left;
- the first digit below `9` increments, and the carry stops there.

If the carry passes the front, every digit was a `9` (e.g. `999`), and the
result is `1` followed by zeros, one digit longer than the input.

```python
def plus_one(digits):
    result = list(digits)
    for i in range(len(result) - 1, -1, -1):
        if result[i] < 9:
            result[i] += 1
            return result
        result[i] = 0
    return [1] + result
```

## Complexity

- Time: O(n) in the worst case (all nines); usually it stops at the last digit.
- Space: O(n) for the copy (the result itself).

## Pitfalls

- Converting to an integer and back overflows: 100 digits is far beyond 64
  bits (Python happens to cope, but that's not the exercise).
- The all-nines case needs a longer array; in fixed-size array languages
  allocate `n + 1` slots (they're zero by default, so only set the first to 1).
- Don't stop after the last digit unconditionally: `[1, 9]` must become
  `[2, 0]`, not `[1, 10]`.
