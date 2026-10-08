# Approach: two pointers

Keep an index `i` into `first` and `j` into `second`. For the current pair
compute `lo = max(first[i][0], second[j][0])` and
`hi = min(first[i][1], second[j][1])`. If `lo <= hi`, the pair overlaps in
`[lo, hi]`, which is part of the answer.

Then advance the interval that ends first. Say `first[i]` ends before
`second[j]`. Every later interval of `second` starts after `second[j]` ends,
which is after `first[i]` ends, so `first[i]` has nothing more to meet. (If the
ends are equal, either one may be dropped.)

The pieces come out in increasing order because both pointers only move right
and the lists are disjoint.

```python
def interval_intersection(first, second):
    result = []
    i = j = 0
    while i < len(first) and j < len(second):
        lo = max(first[i][0], second[j][0])
        hi = min(first[i][1], second[j][1])
        if lo <= hi:
            result.append([lo, hi])
        if first[i][1] < second[j][1]:
            i += 1
        else:
            j += 1
    return result
```

## Complexity

- Time: O(m + n), each step advances one pointer.
- Space: O(1) besides the output.

## Pitfalls

- Testing `lo < hi` instead of `lo <= hi`: a single shared point is a valid
  intersection `[x, x]`.
- Advancing the pointer with the smaller *start* rather than the smaller *end*.
  That can skip an interval that still overlaps the next one.
- Forgetting that either list may be empty.
