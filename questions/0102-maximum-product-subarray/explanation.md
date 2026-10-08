# Approach: track the maximum and minimum product ending here

Unlike sums, a product can flip sign: a very negative product becomes a very large
one after multiplying by a negative number. So for each position we keep two
values — `hi`, the largest product of a stretch ending here, and `lo`, the smallest.

For the next value `x`, a stretch ending at `x` is either `x` alone or an extension
of a stretch ending just before it. If `x` is negative, multiplying swaps the roles
of `hi` and `lo`, so we swap them first. A zero resets both to 0, and the next
element naturally starts fresh because `x` alone is always a candidate.

```python
def max_product(nums):
    best = hi = lo = nums[0]
    for x in nums[1:]:
        if x < 0:
            hi, lo = lo, hi
        hi = max(x, hi * x)
        lo = min(x, lo * x)
        best = max(best, hi)
    return best
```

## Alternative: scan from both ends

Between zeros, the best stretch is always a prefix or a suffix of the zero-free
segment (it drops either everything up to the first negative or everything after
the last one). Taking running products left-to-right and right-to-left, resetting
to 1 after a zero, finds it too.

## Complexity

- Time: O(n) — one pass.
- Space: O(1).

## Pitfalls

- Tracking only the maximum fails on inputs like `[-2, 3, -4]`, where the answer 24
  comes from a negative product times a negative value.
- Initialising `best` to 0 or 1 is wrong for `[-5]`; start from `nums[0]`.
- Zeros split the list: no useful stretch crosses a 0, but 0 itself may be the
  answer when every other product is negative.
