# Approach: greedy on right end points

Sort the balloons by `x_end`. The balloon with the smallest end must be hit by
some arrow, and that arrow cannot be to the right of its end. Moving the arrow
right, up to that end, only gains balloons: any balloon containing a point
further left and still unburst has an end at least as large, so it also
contains the first balloon's end. So place the first arrow at `x_end` of the
first balloon.

Every balloon whose `x_start` is at most the arrow position is burst by it (its
end is at least the arrow position because of the sort). The first balloon
with `x_start > pos` needs a new arrow, placed at its own end by the same
argument.

```python
def find_min_arrow_shots(points):
    ordered = sorted(points, key=lambda p: p[1])
    arrows = 1
    pos = ordered[0][1]
    for start, end in ordered:
        if start > pos:
            arrows += 1
            pos = end
    return arrows
```

## Complexity

- Time: O(n log n) for the sort.
- Space: O(n) for the sorted copy.

## Pitfalls

- Comparators written as `a[1] - b[1]` overflow 32-bit integers when the
  coordinates span the full range. Use `Integer.compare` in Java, `<` in C++.
- Using `start >= pos` for a new arrow. Balloons that only touch at `pos` are
  burst by the same arrow because the ranges are closed.
- Sorting by start and keeping the first balloon's end. You must shrink the
  shared region to the smallest end seen, which sorting by end gives for free.
