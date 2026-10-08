# Approach: upper-bound binary search

Because `letters` is sorted, the letters that are strictly greater than
`target` form a suffix of the array. The answer is the first element of that
suffix, which is exactly the classic *upper bound* position: the first index
`i` with `letters[i] > target`.

Search the half-open range `[lo, hi)`. If `letters[mid] <= target`, neither
`mid` nor anything before it can be the answer, so move `lo` past it.
Otherwise `mid` is a candidate and the answer is at `mid` or to its left.
When the range is empty, `lo` is the upper bound. If it equals `n`, no letter
is greater and the result wraps to `letters[0]`.

```python
def next_greatest_letter(letters, target):
    lo, hi = 0, len(letters)
    while lo < hi:
        mid = (lo + hi) // 2
        if letters[mid] <= target:
            lo = mid + 1
        else:
            hi = mid
    return letters[lo % len(letters)]
```

## Complexity

- Time: O(log n).
- Space: O(1).

## Pitfalls

- Using `<` instead of `<=` in the comparison. That finds the first letter
  *not smaller* than `target`, which returns `target` itself when it is
  present.
- Forgetting the wrap-around: when every letter is `<= target`, the index
  runs off the end of the array.
- Returning `target + 1` as a character. The answer must be a letter that is
  actually in the array.
