# Approach: keep the top three distinct values

Scan once, maintaining `first > second > third`, the three largest distinct
values seen so far, any of which may still be empty. For each `x`:

- if `x` equals one of them, it is a duplicate: skip it;
- if `x > first`, shift `first -> second -> third` and put `x` first;
- else if `x > second`, shift `second -> third` and put `x` second;
- else if `x > third`, replace `third`.

At the end, return `third` if it is filled, otherwise `first`.

```python
def third_max(nums):
    top = []  # distinct values, largest first, at most three
    for x in nums:
        if x in top:
            continue
        top.append(x)
        top.sort(reverse=True)
        if len(top) > 3:
            top.pop()
    return top[2] if len(top) == 3 else top[0]
```

The Python version keeps a list of at most three values; the other languages
use three variables with a sentinel that lies outside the 32-bit range
(`null`, `Long.MIN_VALUE`, `LLONG_MIN`, `math.MinInt64`).

## Complexity

- Time: O(n): each element does constant work.
- Space: O(1).

## Pitfalls

- Counting duplicates: in `[5, 5, 3, 3, 8]` the third maximum is `3`, not
  `5`.
- Using `-2^31` as the "empty" marker: `[1, 2, -2^31]` must return `-2^31`,
  which would be indistinguishable from "no third value".
- Returning the minimum when fewer than three distinct values exist; the
  rule says return the maximum.
