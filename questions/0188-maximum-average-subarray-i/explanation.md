# Approach: fixed-size sliding window

All candidate blocks have length `k`, so comparing averages is the same as
comparing sums. Compute the sum of the first `k` elements, then slide the
window one position at a time: the element at `i` enters, the element at
`i - k` leaves. Keep the best sum seen and divide by `k` once at the end.

```python
def find_max_average(nums, k):
    window = sum(nums[:k])
    best = window
    for i in range(k, len(nums)):
        window += nums[i] - nums[i - k]
        best = max(best, window)
    return best / k
```

Sums stay integers the whole time, so there is no floating-point drift. The
largest possible sum is `10^5 * 10^4 = 10^9`, which still fits in a 32-bit int.

## Complexity

- Time: O(n), each element enters and leaves the window once.
- Space: O(1).

## Pitfalls

- Recomputing every window sum gives O(n * k), too slow when both are large.
- Initialising the best value to `0` is wrong when every number is negative;
  start from the first window's sum.
- Dividing with integer division (`//` in Python, `int / int` in Java, C++
  and Go) truncates the answer. Convert to floating point before dividing.
