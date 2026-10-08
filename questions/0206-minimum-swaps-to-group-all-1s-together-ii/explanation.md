# Approach: fixed-size circular sliding window

Let `c` be the number of `1`s. At the end they occupy some circular window of
`c` consecutive positions. For a chosen window, each `0` inside it pairs with
a `1` outside it (there are exactly as many of each), and one swap fixes one
pair, so the window costs exactly its number of `0`s, i.e. `c - ones_inside`.

Minimising the cost means maximising the `1`s inside a length-`c` window over
all `n` circular starting positions. Slide the window one step at a time:
add the element entering on the right, subtract the one leaving on the left,
wrapping indices with `% n`.

```python
def min_swaps_circular(nums):
    n = len(nums)
    ones = sum(nums)
    if ones == 0:
        return 0
    window = sum(nums[:ones])
    best = window
    for i in range(ones, ones + n - 1):
        window += nums[i % n] - nums[i - ones]
        best = max(best, window)
    return ones - best
```

## Complexity

- Time: O(n).
- Space: O(1).

## Pitfalls

- Ignoring the wrap-around: a linear window misses blocks such as the one in
  Example 2.
- Thinking swaps must be adjacent. Any two positions can be exchanged, which is
  why the cost is just the number of misplaced `0`s.
- An array with no `1`s: the window length is `0`; return `0` directly.
