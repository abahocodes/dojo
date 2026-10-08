# Approach: one pass with a hash set

Scan the array, keeping a set of the values already seen. For the current
value `x`, a valid partner at an earlier index is either `2 * x` or, if `x`
is even, `x / 2`. If either is in the set, answer `true`. Otherwise add `x`
and continue. Every valid pair is found when its later element is scanned,
and because `x` is added only after the check, it never pairs with itself.

```python
def check_if_exist(arr):
    seen = set()
    for x in arr:
        if 2 * x in seen or (x % 2 == 0 and x // 2 in seen):
            return True
        seen.add(x)
    return False
```

## Complexity

- Time: O(n) expected.
- Space: O(n) for the set.

## Pitfalls

- Building the full set first and then testing `2 * x in set`: a single `0`
  finds itself. Either count zeros separately or check before inserting.
- Testing `x / 2` for odd `x`. With integer division, `-3 // 2` is `-2` in
  Python and `-1` in Java/C++/Go, and both are wrong; only even values have
  a half.
