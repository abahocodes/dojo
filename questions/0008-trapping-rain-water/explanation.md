# Approach: two pointers

The water above column `i` is `min(max_left(i), max_right(i)) - height[i]`.
Instead of precomputing both maxima, sweep inward from both ends. Suppose
`left_max <= right_max`. The true right maximum for the left pointer is at
least `right_max`, so the limiting wall for the left column is `left_max` —
we can settle it right away and move on. The mirror argument handles the right.

```python
def trap(height):
    lo, hi = 0, len(height) - 1
    left_max = right_max = 0
    water = 0
    while lo <= hi:
        if left_max <= right_max:
            left_max = max(left_max, height[lo])
            water += left_max - height[lo]
            lo += 1
        else:
            right_max = max(right_max, height[hi])
            water += right_max - height[hi]
            hi -= 1
    return water
```

## Complexity

- Time: O(n) — every column is settled once.
- Space: O(1).

## Pitfalls

- Brute force (scan left and right for every column) is O(n²) and times out on
  the larger cases.
- Updating the running max *before* adding water keeps each term non-negative.
- Alternatives: prefix/suffix max arrays (O(n) space, simplest to get right), or
  a monotonic decreasing stack that fills water layer by layer as each "basin"
  closes.
