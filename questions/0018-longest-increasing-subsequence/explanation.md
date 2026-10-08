# Approach: Patience sorting with binary search (O(n log n))

Maintain a list `tails` such that `tails[k]` holds the smallest tail element among all strictly increasing subsequences of length `k + 1` discovered so far. Because a smaller tail is always at least as good for future extensions, `tails` stays sorted in increasing order.

For each element `x` in `nums`:
1. Binary-search for the **leftmost** index `pos` where `tails[pos] >= x` (i.e. `bisect_left`).
2. If `pos == len(tails)`, no element is `>= x`, so `x` extends the longest subsequence → append.
3. Otherwise, replace `tails[pos]` with `x`. This keeps the tail of that length as small as possible without changing the length.

The length of `tails` at the end equals the LIS length.

```python
import bisect

def longest_increasing_subsequence(nums):
    tails = []
    for x in nums:
        pos = bisect.bisect_left(tails, x)
        if pos == len(tails):
            tails.append(x)
        else:
            tails[pos] = x
    return len(tails)
```

## Complexity

- **Time:** O(n log n) — one binary search per element.
- **Space:** O(n) for the `tails` list (at most n entries).

## Pitfalls

- **Strict vs. non-strict:** Use `bisect_left` (first `>= x`) for *strictly* increasing. Using `bisect_right` (first `> x`) would allow equal elements, giving the *non-decreasing* variant.
- **`tails` is not the LIS itself.** It is a bookkeeping array; the actual subsequence would require back-pointers. The problem only asks for the *length*, so we don't need them.
- **Empty input:** If `nums` is empty, the loop never runs and `tails` stays empty, correctly returning 0.
- **All-equal input** (e.g. `[7,7,7]`): every element replaces `tails[0]`, so the answer is 1, which is correct for *strictly* increasing.
