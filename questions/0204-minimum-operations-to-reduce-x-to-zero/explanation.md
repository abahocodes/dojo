# Approach: longest middle block with a fixed sum

Removing elements from both ends leaves a contiguous middle block. The removed
values add up to `x` exactly when the kept block adds up to
`target = sum(nums) - x`. So the question becomes: what is the longest
subarray whose sum is `target`? The answer is `n` minus that length, or `-1`
if there is no such subarray.

Because every value is positive, the window sum only grows when the right edge
moves and only shrinks when the left edge moves, so a two-pointer sliding
window finds every candidate in one pass.

```python
def min_operations_reduce_x(nums, x):
    target = sum(nums) - x
    if target < 0:
        return -1
    best = -1
    window = 0
    left = 0
    for right, value in enumerate(nums):
        window += value
        while window > target:
            window -= nums[left]
            left += 1
        if window == target:
            best = max(best, right - left + 1)
    return -1 if best == -1 else len(nums) - best
```

When `target == 0` the window shrinks to empty (length 0), which correctly
yields `n` operations: remove everything.

## Complexity

- Time: O(n), each pointer moves at most `n` times.
- Space: O(1).

## Pitfalls

- Trying greedy choices (always take the larger end): it fails easily, e.g.
  when a small element hides a large one behind it.
- Forgetting the `target < 0` case: `x` larger than the total is impossible.
- Forgetting the `target == 0` case, where the empty window is the answer and
  all `n` elements are removed.
- Searching for the shortest removed prefix/suffix with a BFS or DP over both
  ends: correct but quadratic.
