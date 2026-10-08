# Approach: sort, deduplicate, slide a window

An operation can turn an element into anything, so the question is how many
elements can stay untouched. The untouched ones must be distinct and fit in
a range `[x, x + n - 1]` of `n` consecutive integers; the changed elements
then fill the gaps of that range. So:

    answer = n - (most distinct values that fit in a range of width n - 1)

Some optimal range starts at a value we keep (otherwise slide it right until
its left end touches the smallest kept value). Sort the distinct values into
`u`; for each `i`, the range `[u[i], u[i] + n - 1]` covers `u[i..j-1]`, where
`j` is the first index with `u[j] > u[i] + n - 1`. Because `j` only moves
right as `i` does, two pointers find every count in linear time after
sorting.

```python
def min_operations_continuous(nums):
    n = len(nums)
    u = sorted(set(nums))
    best = 0
    j = 0
    for i in range(len(u)):
        while j < len(u) and u[j] <= u[i] + n - 1:
            j += 1
        best = max(best, j - i)
    return n - best
```

## Complexity

- Time: O(n log n) for sorting; the window itself is O(n).
- Space: O(n) for the distinct values.

## Pitfalls

- Keeping duplicates. Two equal values can't both survive, so deduplicate
  before counting, but use the **original** length `n` for the range width
  and the answer.
- Off-by-one in the window: the range holds `n` integers, so its right end is
  `u[i] + n - 1`, not `u[i] + n`.
- In Java/C++, `u[i] + n - 1` stays below 2^31 here, but it is close enough
  that a `long` is a cheap safety net.
