# Approach: put every value in its home slot (cyclic sort)

With `n` numbers, at most `n` distinct values from `1..n` can be present, so
the smallest missing positive lies in `1..n+1`. Everything outside `1..n`
(zero, negatives, large numbers) is irrelevant.

Treat index `v - 1` as the home of value `v`. Walk the array; while the
current value `v` is in range and its home does not already hold `v`, swap it
home. Each swap places one value permanently, so there are at most `n` swaps
in total even though the loop is nested. The check `nums[v - 1] != v` (rather
than `i != v - 1`) stops infinite swapping on duplicates.

After the pass, scan for the first index whose value is not `i + 1`.

```python
def first_missing_positive(nums):
    n = len(nums)
    for i in range(n):
        while 1 <= nums[i] <= n and nums[nums[i] - 1] != nums[i]:
            j = nums[i] - 1
            nums[i], nums[j] = nums[j], nums[i]
    for i in range(n):
        if nums[i] != i + 1:
            return i + 1
    return n + 1
```

## Complexity

- Time: O(n): every swap puts a value in its final slot, so the inner loop
  runs at most `n` times overall.
- Space: O(1) extra; the input array is reused.

## Pitfalls

- Swapping on `i != nums[i] - 1` instead of `nums[nums[i] - 1] != nums[i]`:
  with duplicates such as `[1, 1]` it loops forever.
- In Python, `nums[i], nums[nums[i] - 1] = ...` evaluates the left targets
  in order and uses the already-changed `nums[i]`. Save the index first.
- Computing `nums[i] - 1` on `-2^31` overflows in Java/C++ if done before
  the range check; test `1 <= v <= n` first.
- Returning `n` instead of `n + 1` when every slot is filled.
