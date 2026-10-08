# Approach: sort, then binary search on the distance

Sort `nums`. For a candidate distance `d`, count the pairs with gap `<= d`:
sweep `right` across the array and keep `left` as the first index with
`nums[right] - nums[left] <= d`. Every index in `[left, right)` pairs with
`right`, contributing `right - left` pairs. Because the array is sorted, `left`
only moves forward, so the count takes O(n).

The count is non-decreasing in `d`, so binary search for the smallest `d` with
count `>= k`. That `d` is an actual gap, because the count increases there.

```python
def smallest_distance_pair(nums, k):
    nums = sorted(nums)
    n = len(nums)

    def pairs_within(limit):
        count = left = 0
        for right in range(n):
            while nums[right] - nums[left] > limit:
                left += 1
            count += right - left
        return count

    lo, hi = 0, nums[-1] - nums[0]
    while lo < hi:
        mid = (lo + hi) // 2
        if pairs_within(mid) >= k:
            hi = mid
        else:
            lo = mid + 1
    return lo
```

## Complexity

- Time: O(n log n + n log W), where W is `max(nums) - min(nums)`.
- Space: O(n) for the sorted copy (O(1) if you sort in place).

## Pitfalls

- Generating all pairs is O(n^2) memory and time: 5 * 10^7 pairs at the limit.
- A heap of the closest neighbours works but costs O((n + k) log n), and `k`
  can be in the tens of millions.
- Use `>= k`, not `== k`: duplicate gaps make the count jump over `k`.
- The pair count peaks just under 5 * 10^7, which fits in 32 bits, but a
  64-bit counter costs nothing and survives larger inputs.
