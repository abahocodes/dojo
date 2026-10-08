# Approach: prefix sums + monotonic deque

Let `P[0] = 0` and `P[j+1] = P[j] + nums[j]`. A block `nums[i..j-1]` has sum
`P[j] - P[i]`, so we want the minimum `j - i` over pairs `i < j` with
`P[j] - P[i] >= k`.

Two observations prune the candidate starts `i`:

1. **Dominated starts.** If `i1 < i2` and `P[i1] >= P[i2]`, any `j` that
   works with `i1` also works with `i2` and gives a shorter block. So we keep
   only starts whose prefix sums strictly increase: before pushing `j`, pop
   from the back every start with `P[back] >= P[j]`.
2. **Used-up starts.** Once start `i` is matched with some `j`, later ends only
   give longer blocks for `i`, so it can be removed from the front.

Because the deque's prefix sums increase, checking the front is enough: if the
front does not satisfy `P[j] - P[front] >= k`, no other start does.

```python
from collections import deque

def shortest_subarray(nums, k):
    prefix = [0]
    for x in nums:
        prefix.append(prefix[-1] + x)
    best = len(nums) + 1
    starts = deque()
    for j, p in enumerate(prefix):
        while starts and p - prefix[starts[0]] >= k:
            best = min(best, j - starts.popleft())
        while starts and prefix[starts[-1]] >= p:
            starts.pop()
        starts.append(j)
    return best if best <= len(nums) else -1
```

## Complexity

- Time: O(n). Every index enters and leaves the deque at most once.
- Space: O(n) for the prefix sums and the deque.

## Pitfalls

- A classic shrinking sliding window. It assumes that extending a window
  never lowers its sum, which is false with negative numbers.
- Overflow: prefix sums reach `10^5 * 10^5 = 10^10`, beyond 32 bits. Store
  them as 64-bit integers.
- Popping from the back with `>` instead of `>=`. Equal prefix sums also make
  the earlier start useless.
- Returning `0` instead of `-1` when no block qualifies.
