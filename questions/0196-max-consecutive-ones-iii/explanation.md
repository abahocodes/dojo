# Approach: sliding window with a zero budget

Flipping at most `k` zeros and then taking a run of ones is the same as picking
a contiguous subarray with at most `k` zeros in it. That property is monotone:
if a window has at most `k` zeros, so does every window inside it. So a
two-pointer window works. Grow the right edge one element at a time. When the
window holds more than `k` zeros, shrink it from the left until it holds `k`
again. After each step the window is the longest valid one ending at `right`,
so the best of those is the answer.

```python
def longest_ones(nums, k):
    left = zeros = best = 0
    for right, x in enumerate(nums):
        if x == 0:
            zeros += 1
        while zeros > k:
            if nums[left] == 0:
                zeros -= 1
            left += 1
        best = max(best, right - left + 1)
    return best
```

## Complexity

- Time: O(n). Each index enters and leaves the window at most once.
- Space: O(1).

## Pitfalls

- Restarting the window from scratch at every zero gives O(n * k) or worse.
- With `k = 0` the answer is the longest existing run of ones, which may be 0.
- With `k >= ` the number of zeros, the answer is the whole array.
