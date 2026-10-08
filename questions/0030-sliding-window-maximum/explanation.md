# Approach: monotonic deque

Store indices in a deque so that their values decrease from front to back. The
front is always the maximum of the current frame.

For each index `i`:

1. Pop indices from the back while their value is `<= nums[i]` — they are
   dominated by a newer, at-least-as-large element.
2. Append `i`.
3. If the front index is `<= i - k`, it has left the frame; pop it.
4. Once `i >= k - 1`, record `nums[deque[0]]`.

```python
from collections import deque

def max_sliding_window(nums, k):
    dq = deque()
    out = []
    for i, x in enumerate(nums):
        while dq and nums[dq[-1]] <= x:
            dq.pop()
        dq.append(i)
        if dq[0] <= i - k:
            dq.popleft()
        if i >= k - 1:
            out.append(nums[dq[0]])
    return out
```

## Complexity

- Time: O(n) — every index is pushed once and popped at most once.
- Space: O(k) for the deque, plus the output.

## Pitfalls

- Store indices, not values: you need positions to know when the front expires.
- Expire at most one index per step from the front; since indices enter in order,
  only the front can be out of range.
- A heap also works in O(n log n) with lazy deletion, but it is slower and easy
  to get wrong when old maxima linger in the heap.
