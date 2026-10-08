# Approach: meet in the middle with a hash map

Rewrite the condition as `nums1[i] + nums2[j] == -(nums3[k] + nums4[l])`. The
left side only depends on the first two arrays, the right side only on the
last two, so the two halves can be enumerated independently.

1. For every pair `(i, j)`, count how often each sum `nums1[i] + nums2[j]`
   occurs.
2. For every pair `(k, l)`, look up how many first-half pairs have the sum
   `-(nums3[k] + nums4[l])` and add that number to the answer.

Each matching combination of a first-half pair and a second-half pair is
exactly one valid tuple, so the total is the answer.

```python
def four_sum_count(nums1, nums2, nums3, nums4):
    sums = {}
    for a in nums1:
        for b in nums2:
            sums[a + b] = sums.get(a + b, 0) + 1
    count = 0
    for c in nums3:
        for d in nums4:
            count += sums.get(-(c + d), 0)
    return count
```

## Complexity

- Time: O(n^2): two double loops with O(1) hash-map work each.
- Space: O(n^2) for the map of pair sums.

## Pitfalls

- Storing a set of sums instead of counts: equal sums from different index
  pairs are different tuples and must all be counted.
- Overflow worries: each value is at most `2^28` in magnitude, so any sum of
  four fits in a 32-bit integer, and the count is at most `200^4 = 1.6 * 10^9`,
  which also fits.
- Splitting the arrays as 1 + 3 instead of 2 + 2 still costs `O(n^3)`.
