# Approach: monotonic stack over `nums2` + lookup map

Process `nums2` from left to right, keeping a stack of values that are still
waiting for a greater value to their right. The stack is always decreasing
from bottom to top. If a smaller value sat above a larger one, the smaller one
would already have been resolved.

When the next value `x` arrives, every value on top of the stack that is
smaller than `x` has found its answer: pop it and record `next[value] = x`.
Then push `x`. Values still on the stack at the end have no greater value to
their right.

Because values are distinct, a map from value to answer is unambiguous, and
`nums1` is answered with one lookup per element.

```python
def next_greater_element(nums1, nums2):
    nxt = {}
    stack = []
    for x in nums2:
        while stack and stack[-1] < x:
            nxt[stack.pop()] = x
        stack.append(x)
    return [nxt.get(x, -1) for x in nums1]
```

## Complexity

- Time: O(m + n) for `m = len(nums2)` and `n = len(nums1)`. Each value is
  pushed and popped at most once.
- Space: O(m) for the stack and the map.

## Pitfalls

- The search starts at `x`'s position in `nums2`, not at index `i` of
  `nums1`.
- "Greater" means strictly greater. With distinct values this only matters if
  you compare a value with itself.
- Scanning right from each position for every query is O(n * m). That passes
  here but misses the point of the problem.
- Keep the output in `nums1`'s order. Don't iterate over the map.
