# Approach: monotonic stack over two laps

Keep a stack of indices whose next greater element has not been found yet.
Their values are non-increasing from bottom to top: if a bigger value had
arrived after a smaller one, it would already have answered it. When a value
`x` arrives, pop every waiting index whose value is less than `x` and record
`x` as its answer.

Going around the circle once more covers the elements that come "after" an
index by wrapping. Iterate `j` from `0` to `2n - 1` and look at
`nums[j % n]`; push an index only on the first lap, so every index waits at
most once. Indices still on the stack at the end (the maximum, and any value
equal to it) have no greater element and keep `-1`.

```python
def next_greater_circular(nums):
    n = len(nums)
    result = [-1] * n
    stack = []
    for j in range(2 * n):
        x = nums[j % n]
        while stack and nums[stack[-1]] < x:
            result[stack.pop()] = x
        if j < n:
            stack.append(j)
    return result
```

## Complexity

- Time: O(n): `2n` steps, and each index is pushed and popped at most once.
- Space: O(n) for the stack and the result.

## Pitfalls

- Using `<=` when popping, which treats an equal value as greater.
- Running only one lap. The last elements then never see the values at the
  start of the array, e.g. `[2, 7, 3, 5]` would give `-1` for `5`.
- Brute force (walk up to `n - 1` steps from every index) is O(n^2), which is
  slow when most values have a far-away or missing answer.
- Returning `0` or leaving garbage for indices without an answer instead of
  `-1`.
