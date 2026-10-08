# Approach: greedy two-pointer expansion from `k`

Every good subarray contains index `k`, so it can be built by starting from the
single element `[k, k]` and repeatedly adding one neighbour on the left or the
right.

Greedy rule: always absorb the **larger** of the two neighbours `nums[i-1]` and
`nums[j+1]`. For each possible length `L`, this produces the window of length
`L` around `k` with the highest possible minimum. Why: suppose the best window
of length `L` has minimum `m`. Every element in it is `>= m`, and the greedy
walk only ever steps over a value `< m` if both available neighbours are `< m` -
which means the current window is already bounded on both sides by values
smaller than `m`, so no window containing `k` with minimum `m` could be larger
than it. Hence the greedy window at length `L` has minimum `>= m`.

Taking the best `minimum * length` over all `n` window sizes gives the answer.

```python
def maximum_score(nums, k):
    n = len(nums)
    i = j = k
    low = nums[k]
    best = low
    while i > 0 or j < n - 1:
        if i == 0 or (j < n - 1 and nums[j + 1] > nums[i - 1]):
            j += 1
            low = min(low, nums[j])
        else:
            i -= 1
            low = min(low, nums[i])
        best = max(best, low * (j - i + 1))
    return best
```

An equivalent alternative is the "largest rectangle in a histogram" method:
use a monotonic stack to find, for each index `p`, the nearest strictly
smaller element on each side; the widest window where `nums[p]` is the minimum
is valid only if it contains `k`.

## Complexity

- Time: O(n) - each step grows the window by one element.
- Space: O(1).

## Pitfalls

- Forgetting that one side may be exhausted: when `i == 0` you must extend
  right, and when `j == n - 1` you must extend left.
- Ties between the neighbours can go either way; both choices are correct.
- The product reaches `2 * 10^9`, close to the 32-bit limit - compute it in a
  type that does not overflow before comparing (it still fits in a signed
  32-bit `int`).
