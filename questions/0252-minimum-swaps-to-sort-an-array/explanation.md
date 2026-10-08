# Approach: count permutation cycles

Let `order[k]` be the index (in `nums`) of the `k`-th smallest value. In the
sorted array, position `k` must hold `nums[order[k]]`, so the map
`k -> order[k]` is a permutation of the positions. Like every permutation, it
breaks into disjoint cycles.

Inside a cycle of length `L`, each exchange can put at most one value into its
final place, except the last one, which places two. So a cycle needs exactly
`L - 1` exchanges, and exchanges inside different cycles never help each
other. The answer is `sum(L - 1)` = `n - (number of cycles)`.

```python
def min_swaps_to_sort(nums):
    n = len(nums)
    order = sorted(range(n), key=nums.__getitem__)
    seen = [False] * n
    swaps = 0
    for i in range(n):
        length = 0
        j = i
        while not seen[j]:
            seen[j] = True
            j = order[j]
            length += 1
        if length:
            swaps += length - 1
    return swaps
```

## Complexity

- Time: O(n log n) for the sort; the cycle walk is O(n).
- Space: O(n) for `order` and `seen`.

## Pitfalls

- Counting misplaced elements: `[2, 1]` has two misplaced values but needs
  only one exchange.
- Counting adjacent swaps (inversions). Exchanges here may be between any two
  positions, so `[3, 2, 1]` needs 1, not 3.
- Walking a cycle again from each of its members. Mark positions as visited.
- Using values as indices: the values are arbitrary distinct integers, not a
  permutation of `0..n-1`.
