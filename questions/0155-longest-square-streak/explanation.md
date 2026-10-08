# Approach: hash set and forced chains

Duplicates never help (a streak is strictly increasing), so store the values
in a set. Starting from a value `x`, the only possible streak is
`x, x^2, x^4, ...`, continued while the next square is in the set. With
values in `[2, 10^5]` the chain `2, 4, 16, 256, 65536` is the longest
possible, so each walk takes at most 5 steps and trying every value as a
start is linear.

```python
def longest_square_streak(nums):
    present = set(nums)
    best = -1
    for x in present:
        length = 1
        cur = x
        while cur * cur in present:
            cur *= cur
            length += 1
        if length >= 2 and length > best:
            best = length
    return best
```

(Sorting the values and building `dp[x] = dp[sqrt(x)] + 1` from small to
large also works in O(n log n).)

## Complexity

- Time: O(n): building the set, then at most 5 lookups per value.
- Space: O(n) for the set.

## Pitfalls

- Squaring in 32-bit integers: `100000 * 100000` overflows a 32-bit
  `int` in Java and C++. Compare against the maximum value (or use a
  64-bit type) before looking up the square.
- Returning 1 for an array with no streak: a single element does not count,
  the answer is `-1`.
- Counting duplicates twice: `[4, 4]` has no streak.
