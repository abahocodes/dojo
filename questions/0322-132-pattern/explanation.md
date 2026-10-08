# Approach: right-to-left monotonic stack

Scan from the right, looking for a "1". Maintain:

- `third`: the largest value seen so far that has a strictly larger value
  to its left (but still to the right of the current position). Any such
  value can be the `nums[k]`, with that larger value as `nums[j]`.
- a stack of scanned values that is decreasing from bottom to top, holding
  candidates for `nums[j]`.

For each new value `x` (moving left):

1. If `x < third`, then `x`, the "3" for `third`, and `third` form a 132
   pattern.
2. Otherwise `x` might be a new "3": every stack value smaller than `x` lies
   to the right of `x` and can serve as a "2" for it, so pop them and raise
   `third` to the last popped (the largest of them, since the stack is
   decreasing towards the top).
3. Push `x`.

`third` only ever grows, and keeping the largest possible "2" is always the
best choice for finding a "1" later.

```python
def find_132_pattern(nums):
    third = float("-inf")
    stack = []
    for x in reversed(nums):
        if x < third:
            return True
        while stack and stack[-1] < x:
            third = stack.pop()
        stack.append(x)
    return False
```

## Complexity

- Time: O(n): each value is pushed and popped at most once.
- Space: O(n) for the stack.

## Pitfalls

- Using `<=` anywhere: all three comparisons are strict, so equal values never
  form a pattern (`[1, 2, 2]` and `[2, 3, 2]` are `false`).
- Scanning left to right with a stack of maxima: it is much harder to
  remember which "2" candidates are still valid.
- Using `INT_MIN` as the initial `third` with a `<=` test. With the strict
  test and values above `-2^31`, the sentinel is safe.
- The O(n^2) prefix-minimum approach is correct but too slow for
  `n = 2 * 10^5`.
