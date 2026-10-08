# Approach: running maximum and running minimum

An element `nums[i]` has to be inside the sorted window if some earlier
element is larger than it (it must move left) or some later element is
smaller than it (it must move right). Everything outside the window is
already in its final place.

- Scanning left to right with the running maximum, the **last** index where
  `nums[i] < max_so_far` is the right end of the window.
- Scanning right to left with the running minimum, the **last** index (the
  leftmost) where `nums[i] > min_so_far` is the left end.

Every element between those ends belongs to the window too, since the window
is contiguous.

```python
def find_unsorted_subarray(nums):
    n = len(nums)
    end = -1
    running_max = nums[0]
    for i in range(1, n):
        if nums[i] < running_max:
            end = i
        else:
            running_max = nums[i]
    if end == -1:
        return 0
    start = n
    running_min = nums[-1]
    for i in range(n - 2, -1, -1):
        if nums[i] > running_min:
            start = i
        else:
            running_min = nums[i]
    return end - start + 1
```

Comparing against `sorted(nums)` and taking the first and last mismatch is a
correct O(n log n) alternative.

## Complexity

- Time: O(n), two passes.
- Space: O(1).

## Pitfalls

- Only looking at adjacent "descents". In `[1, 3, 3, 2, 2, 4]` the only
  descent is between the second 3 and the first 2, but both 3s and both 2s
  must be in the window (length 4).
- Using `<=` against the running maximum: equal values are already in
  order, and counting them would make the window too long.
- Returning `end - start + 1` for a sorted array where neither bound was set.
  Handle the already-sorted case explicitly.
