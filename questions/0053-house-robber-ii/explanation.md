# Approach: break the circle, solve two straight streets

In any valid plan, house `0` and house `n - 1` are not both robbed. So every
valid plan is a valid plan for one of two straight streets:

- `nums[0 .. n-2]` (the last house is skipped), or
- `nums[1 .. n-1]` (the first house is skipped).

Conversely, any plan on either straight street is valid on the circle, because
it can't use both ends. The answer is the better of the two linear answers,
each computed with the usual two-variable DP.

```python
def rob_circular(nums):
    def line(lo, hi):  # best for nums[lo:hi] as a straight street
        prev, curr = 0, 0
        for i in range(lo, hi):
            prev, curr = curr, max(curr, prev + nums[i])
        return curr

    n = len(nums)
    if n == 1:
        return nums[0]
    return max(line(0, n - 1), line(1, n))
```

## Complexity

- Time: O(n), two linear passes.
- Space: O(1) (iterating over index ranges instead of slicing).

## Pitfalls

- With a single house both ranges are empty, so return `nums[0]` directly.
- Running the linear DP once over the whole array ignores the wrap-around and
  may rob both ends, e.g. `[3, 5, 3]` would give `6`.
- Two houses are adjacent both ways round; the answer is simply the larger one.
- Exponential "try every subset" search is hopeless at 1000 houses.
