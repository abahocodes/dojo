# Approach: binary search on the answer

Define `pieces(C)` as the fewest contiguous pieces whose sums all stay at or
below `C`. A greedy left-to-right scan computes it: add elements to the current
piece until the next one would push it over `C`, then start a new piece.
Closing a piece later never hurts, so greedy is optimal.

If `pieces(C) <= k`, a cut into exactly `k` pieces with load at most `C` also
exists: split any piece with two or more elements (possible because
`k <= len(nums)`), which only lowers sums. `pieces(C)` never grows as `C` grows,
so the smallest `C` with `pieces(C) <= k` can be found by binary search over
`[max(nums), sum(nums)]`.

```python
def split_array(nums, k):
    def pieces_needed(cap):
        pieces, current = 1, 0
        for x in nums:
            if current + x > cap:
                pieces += 1
                current = x
            else:
                current += x
        return pieces

    lo, hi = max(nums), sum(nums)
    while lo < hi:
        mid = (lo + hi) // 2
        if pieces_needed(mid) <= k:
            hi = mid
        else:
            lo = mid + 1
    return lo
```

## Complexity

- Time: O(n log S), where S is `sum(nums)`: each of the ~log S probes scans
  the array once.
- Space: O(1) beyond the input.

## Pitfalls

- The lower bound must be `max(nums)`, not 0: with a smaller cap the greedy
  scan would try to place an element that alone exceeds the cap.
- Asking for exactly `k` pieces is the same as at most `k` pieces; don't
  require `pieces(C) == k`, which is not monotone.
- A dynamic program over (pieces, prefix) also works but costs O(k n^2), far
  too slow for `n = 10^4`.
- Zeros are allowed; they never force a new piece.
