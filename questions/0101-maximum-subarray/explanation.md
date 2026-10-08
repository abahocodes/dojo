# Approach: Kadane's algorithm

Let `current` be the largest total of a stretch that ends at the element we are
looking at. For the next element `x` there are only two candidates: extend the
previous best stretch (`current + x`) or start a new one containing just `x`.
A negative running total can only hurt whatever follows, so we drop it.

The answer is the largest `current` seen anywhere.

```python
def max_sub_array(nums):
    current = best = nums[0]
    for x in nums[1:]:
        current = max(x, current + x)
        best = max(best, current)
    return best
```

## Complexity

- Time: O(n) — one pass.
- Space: O(1).

## Pitfalls

- Initialising `best` to 0 returns 0 for an all-negative list, but the stretch must
  be non-empty — start from `nums[0]` instead.
- Resetting `current` to 0 (rather than to `x`) has the same problem.
- A divide-and-conquer solution runs in O(n log n); it's a nice follow-up, but
  Kadane is both simpler and faster.
