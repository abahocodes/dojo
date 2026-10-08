# Approach: sorted prefix + sorted suffix, joined with two pointers

The kept elements are a prefix `arr[:i+1]` followed by a suffix `arr[j:]`
(either may be empty). The prefix must be non-decreasing, so it lies inside
the longest non-decreasing prefix of `arr`; likewise the suffix lies inside
the longest non-decreasing suffix, which starts at some index `right`. The
join is valid when the last kept prefix value is at most the first kept
suffix value.

1. Find `right`. If it is `0` the array is already sorted: return 0.
2. Start with the answer `right` (remove the whole front part).
3. Walk `left` through the sorted prefix. For each `left`, move `right`
   forward until `arr[right] >= arr[left]`; then removing `arr[left+1:right]`
   (length `right - left - 1`) is valid. Because `arr[left]` never decreases,
   `right` never needs to move back.

```python
def find_length_of_shortest_subarray(arr):
    n = len(arr)
    right = n - 1
    while right > 0 and arr[right - 1] <= arr[right]:
        right -= 1
    if right == 0:
        return 0
    best = right                      # keep only the suffix
    for left in range(n):
        if left > 0 and arr[left - 1] > arr[left]:
            break                     # prefix no longer sorted
        while right < n and arr[right] < arr[left]:
            right += 1
        best = min(best, right - left - 1)   # right == n keeps only the prefix
    return best
```

## Complexity

- Time: O(n): `left` and `right` each move forward at most `n` times.
- Space: O(1).

## Pitfalls

- Removing more than one block. Only one contiguous piece may go, so the kept
  part is always prefix + suffix.
- Forgetting the "keep only the prefix" option. It appears naturally when
  `right` reaches `n`, which gives `n - left - 1`.
- Using `<` instead of `<=` when growing the sorted runs: equal neighbours are
  allowed in a non-decreasing sequence.
- Moving `left` past the end of the sorted prefix. Stop as soon as the prefix
  itself breaks.
