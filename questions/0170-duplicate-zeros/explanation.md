# Approach: count zeros, then fill from the right

Element `i` ends up at `i + shift`, where `shift` is the number of zeros
strictly before it (a zero also produces a second copy at `i + shift + 1`).
Anything whose target is `>= n` falls off.

Writing from right to left is safe: every target is at or to the right of the
source, and positions to the right of `i` have already been read. Start with
`shift` equal to the total number of zeros and decrement it whenever the walk
passes a zero.

The function returns a new array, so the solutions copy `arr` first and run
the in-place algorithm on the copy.

```python
def duplicate_zeros(arr: list[int]) -> list[int]:
    out = list(arr)
    n = len(out)
    # shift = number of zeros strictly before index i: out[i] lands at i + shift.
    shift = out.count(0)
    for i in range(n - 1, -1, -1):
        if out[i] == 0:
            shift -= 1
            if i + shift + 1 < n:
                out[i + shift + 1] = 0
        if i + shift < n:
            out[i + shift] = out[i]
    return out
```

## Complexity

- Time: O(n): one pass to count, one to fill.
- Space: O(1) beyond the output array.

## Pitfalls

- Shifting left to right in place: you overwrite elements before moving them.
- A zero whose first copy fits but whose second copy falls off the end:
  write only the copy that is inside the array.
- Decrementing `shift` after placing a zero instead of before: the zero's own
  position must not count itself.
