# Approach: count one array, consume with the other

Count how many times each value occurs in `nums1`. Then walk `nums2`: each
element takes one remaining copy if there is one, and every successful take
goes into the answer. A value ends up in the output `min(a, b)` times because
the takes stop either when `nums1`'s copies run out or when `nums2` has no
more of it.

```python
def intersect(nums1, nums2):
    counts = [0] * 1001
    for x in nums1:
        counts[x] += 1
    out = []
    for x in nums2:
        if counts[x] > 0:
            counts[x] -= 1
            out.append(x)
    return out
```

With an unbounded value range, use a hash map instead of the array. If both
arrays were already sorted, two pointers would do it with O(1) extra space.

## Complexity

- Time: O(n + m).
- Space: O(1) for the 1001 counters, plus the output.

## Pitfalls

- Using a set, which loses multiplicity (`[4, 4]` would become `[4]`).
- Forgetting to decrement the count, which outputs a value as many times as it
  appears in `nums2`.
- Removing elements from a list as you match them, which is O(n * m).
