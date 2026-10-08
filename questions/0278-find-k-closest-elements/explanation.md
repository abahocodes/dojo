# Approach: binary search for the window start

In a sorted array the `k` closest elements form a contiguous window, so we
only need its left edge `left` in the range `[0, n - k]`.

Compare the window starting at `mid` with the one starting at `mid + 1`. They
share everything except `arr[mid]` and `arr[mid + k]`. Since
`arr[mid] <= arr[mid + k]`:

- If `x - arr[mid] > arr[mid + k] - x`, then `arr[mid + k]` is strictly
  closer than `arr[mid]` (this also covers `x` lying beyond `arr[mid + k]`).
  Shifting right helps, and so the answer starts at `mid + 1` or later.
- Otherwise `arr[mid]` is at least as close, winning ties because it is the
  smaller value, so the answer starts at `mid` or earlier.

The comparison is monotone in `mid`, which is what makes the binary search
valid. Comparing signed differences instead of absolute values matters: it
handles duplicates correctly where `abs` comparisons can pick the wrong side.

```python
def find_closest_elements(arr, k, x):
    lo, hi = 0, len(arr) - k
    while lo < hi:
        mid = (lo + hi) // 2
        if x - arr[mid] > arr[mid + k] - x:
            lo = mid + 1
        else:
            hi = mid
    return arr[lo:lo + k]
```

## Complexity

- Time: O(log(n - k) + k), the search plus copying the result.
- Space: O(1) besides the output.

## Pitfalls

- Using `abs(x - arr[mid]) > abs(arr[mid + k] - x)`: with duplicates such as
  `arr = [1, 1, 2, 2, 2, 2, 2, 3, 3]`, `k = 3`, `x = 3`, both sides can look
  equal and the search drifts left of the correct window.
- Searching `hi` up to `n - 1` instead of `n - k`, which reads past the end
  through `arr[mid + k]`.
- Breaking ties toward the larger element. Equal distances favour the smaller
  value.
