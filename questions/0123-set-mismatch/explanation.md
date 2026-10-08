# Approach: seen-array for the duplicate, arithmetic for the missing value

Scan once, marking values in a boolean array; the value found already marked
is the duplicate. Along the way add up all values. The intact array would sum
to `n(n + 1)/2`; the damaged one replaced `missing` with `duplicated`, so

`missing = n(n + 1)/2 - (sum(nums) - duplicated)`.

```python
def find_error_nums(nums):
    seen = [False] * (len(nums) + 1)
    dup = 0
    total = 0
    for x in nums:
        if seen[x]:
            dup = x
        seen[x] = True
        total += x
    n = len(nums)
    missing = n * (n + 1) // 2 - (total - dup)
    return [dup, missing]
```

Alternatives: sign-marking in place (as in "Find All Numbers Disappeared in
an Array") gives O(1) extra space, and an XOR trick does too.

## Complexity

- Time: O(n), one pass.
- Space: O(n) for the seen array.

## Pitfalls

- Returning the pair in the wrong order; the duplicate comes first.
- Assuming the array is sorted, so that the duplicate sits next to itself.
- Overflow when computing `n(n + 1)/2` in a 32-bit type for larger `n`; it
  is safe here, but using a 64-bit sum costs nothing.
