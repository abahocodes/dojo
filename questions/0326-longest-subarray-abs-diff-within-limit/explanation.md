# Approach: sliding window with monotonic deques

Any sub-window of a valid window is valid, so for each right end `r` the best
left end only ever moves rightwards. Sweep `r` across the array and keep `left`
as small as possible while `max(window) - min(window) <= limit`.

To read the window's max and min in O(1):

- `maxq` holds indices whose values are **decreasing** from front to back. An
  index whose value is smaller than a newer value can never be the maximum
  again, so it is popped from the back. The front is the window maximum.
- `minq` is the mirror image, with **increasing** values.

When the window is too spread out, advance `left` and discard front indices that
are now outside the window.

```python
from collections import deque

def longest_subarray_limit(nums, limit):
    maxq, minq = deque(), deque()
    left = 0
    best = 0
    for right, x in enumerate(nums):
        while maxq and nums[maxq[-1]] < x:
            maxq.pop()
        maxq.append(right)
        while minq and nums[minq[-1]] > x:
            minq.pop()
        minq.append(right)
        while nums[maxq[0]] - nums[minq[0]] > limit:
            left += 1
            if maxq[0] < left:
                maxq.popleft()
            if minq[0] < left:
                minq.popleft()
        best = max(best, right - left + 1)
    return best
```

## Complexity

- Time: O(n) — every index enters and leaves each deque at most once, and
  `left` only moves forward.
- Space: O(n) for the deques in the worst case.

## Pitfalls

- Storing values instead of indices makes it impossible to tell when the front
  element has left the window (duplicates make this especially error-prone).
- Popping the front when `left` passes it must use the index, not compare
  values.
- A single element always forms a valid window (`limit >= 0`), so the answer is
  at least 1.
