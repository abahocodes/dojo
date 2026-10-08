# Approach: greedy furthest reach

Reachable indices form a prefix `0..furthest`: any jump from a reachable index can
land on every index up to its maximum distance. So scan left to right, keeping the
furthest reachable index. If the scan reaches an index past `furthest`, there is a
gap you can never cross. Once `furthest` covers the last index, you're done.

```python
def can_jump(nums):
    furthest = 0
    last = len(nums) - 1
    for i, step in enumerate(nums):
        if i > furthest:
            return False
        furthest = max(furthest, i + step)
        if furthest >= last:
            return True
    return True
```

## Alternative: backwards

Scan from the end keeping `goal`, the leftmost index known to reach the last one.
Index `i` can reach the goal when `i + nums[i] >= goal`; the answer is whether
`goal` ends at 0.

## Complexity

- Time: O(n) — a single pass.
- Space: O(1).

## Pitfalls

- Trying every jump length (DFS or DP over all choices) is O(n · max(nums)) and too
  slow when values are large.
- A single-element list is already at its last index: the answer is `true` even
  when `nums[0] == 0`.
- Values can be much larger than the list; reaching *past* the end still counts.
