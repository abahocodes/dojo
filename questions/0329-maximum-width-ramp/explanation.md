# Approach: decreasing stack of left-end candidates

**Candidate left ends.** An index `b` can be ignored as a left end if some
earlier index `a` has `nums[a] <= nums[b]`: every ramp starting at `b` also
works from `a`, and is wider. The surviving candidates are exactly the indices
where a new strict prefix minimum appears; scanning left to right, push `i`
whenever `nums[i]` is smaller than the value at the stack top.

**Matching right ends.** Now scan `j` from `n - 1` down to `0`. While the
candidate on top of the stack satisfies `nums[top] <= nums[j]`, the pair
`(top, j)` is a ramp, and it is the widest ramp that left end can ever get,
because remaining `j` values only get smaller. Record `j - top` and pop it, then
look at the next candidate (further left, and with a larger value).

```python
def max_width_ramp(nums):
    stack = []
    for i, x in enumerate(nums):
        if not stack or x < nums[stack[-1]]:
            stack.append(i)
    best = 0
    for j in range(len(nums) - 1, -1, -1):
        while stack and nums[stack[-1]] <= nums[j]:
            best = max(best, j - stack.pop())
    return best
```

An alternative is to sort indices by value (ties by index) and track the
smallest index seen so far, which is O(n log n).

## Complexity

- Time: O(n) — each candidate is pushed and popped at most once.
- Space: O(n) for the stack.

## Pitfalls

- The condition is `<=`: equal values form a ramp.
- Pushing every index (not only new strict minima) breaks the argument that
  the stack is sorted by value, and the second pass stops too early.
- The second pass must run from the right; scanning `j` forward cannot pop
  candidates safely.
