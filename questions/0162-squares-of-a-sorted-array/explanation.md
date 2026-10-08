# Approach: two pointers from the ends

The squares of a sorted array form a "valley": they decrease through the
negative part and increase through the non-negative part. The largest
square is therefore always at one of the two ends. Repeatedly take the
larger end, write its square at the back of the output, and move that end
inward.

```python
def sorted_squares(nums):
    n = len(nums)
    out = [0] * n
    lo, hi = 0, n - 1
    for w in range(n - 1, -1, -1):
        if abs(nums[lo]) > abs(nums[hi]):
            out[w] = nums[lo] * nums[lo]
            lo += 1
        else:
            out[w] = nums[hi] * nums[hi]
            hi -= 1
    return out
```

## Complexity

- Time: O(n), each element is squared once.
- Space: O(1) beyond the output array.

## Pitfalls

- Filling from the front: the *smallest* square can be anywhere in the
  middle, so you would first have to find the zero crossing.
- Squaring and sorting is correct but O(n log n).
- Squares go up to `10^8`, which still fits in a 32-bit int; with larger
  bounds you would need 64-bit values.
