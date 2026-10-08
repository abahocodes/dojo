# Approach: sort, fix the longest side, two pointers

After sorting, a triple `a <= b <= c` is a triangle exactly when `a + b > c`;
the other two inequalities hold automatically because `c` is the largest.

Fix `k` as the index of the longest side and look for pairs `i < j < k` with
`a[i] + a[j] > a[k]`. Start `i` at `0` and `j` at `k - 1`:

- If `a[i] + a[j] > a[k]`, then because the array is sorted every index
  between `i` and `j - 1` also pairs with `j`. That is `j - i` pairs at once.
  Then `j` is done, so move it left.
- Otherwise `a[i]` is too small even with the biggest partner left, so move
  `i` right.

Each `k` costs one linear sweep.

```python
def triangle_number(nums):
    a = sorted(nums)
    count = 0
    for k in range(len(a) - 1, 1, -1):
        i, j = 0, k - 1
        while i < j:
            if a[i] + a[j] > a[k]:
                count += j - i
                j -= 1
            else:
                i += 1
    return count
```

## Complexity

- Time: O(n^2), sorting plus one O(k) sweep per `k`.
- Space: O(n) for the sorted copy (O(1) extra if you sort in place).

## Pitfalls

- Using `>=` instead of `>`: `(1, 1, 2)` is degenerate (a flat line) and
  must not count.
- Zeros: a side of length 0 can never be part of a triangle. The strict
  inequality already handles this; no special case is needed.
- Fixing the smallest side instead of the largest makes the two-pointer
  movement ambiguous. Anchor the side that the inequality compares against.
- The count can reach about 1.66 * 10^8 for 1000 elements, which still fits
  in a 32-bit integer.
