# Approach: sort and pair neighbours

The maximum element never scores, whatever it is paired with. Its partner
scores, so the partner should be as large as possible: the second largest.
Removing those two leaves the same problem on `2n - 2` elements, and the same
argument applies. Unrolling it, after sorting we pair index `0` with `1`, `2`
with `3`, and so on, and the score is the sum of the even-indexed elements.

Another way to see it: every pair `(a, b)` with `a <= b` loses `b - a`
compared with scoring both values, and pairing neighbours in sorted order
minimises the total of those gaps.

```python
def array_pair_sum(nums):
    return sum(sorted(nums)[::2])
```

Because values are bounded by `10^4`, a counting sort over the `20001`
possible values would make this linear, but a comparison sort is fast enough.

## Complexity

- Time: O(n log n) for the sort.
- Space: O(n) for the sorted copy.

## Pitfalls

- Greedily pairing the smallest with the largest: on `[1, 2, 3, 4]` that
  gives `1 + 2 = 3` instead of `4`.
- Forgetting negative values: the sorted-neighbours rule still holds, and the
  answer itself may be negative.
