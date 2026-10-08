# Approach: hash set of numeric prefixes

The common prefix of two decimal strings is a prefix of each of them, and every
decimal prefix of a number `x` is `x` with trailing digits removed, i.e. one of
`x, x // 10, x // 100, ...`. Insert all of those for every `x` in `arr1` into a
set. Then, for every `y` in `arr2`, strip trailing digits until the value is in
the set; the longest value found (in digits) over all `y` is the answer.

```python
def longest_common_prefix_numbers(arr1, arr2):
    prefixes = set()
    for x in arr1:
        while x > 0 and x not in prefixes:
            prefixes.add(x)
            x //= 10
    best = 0
    for y in arr2:
        while y > 0 and y not in prefixes:
            y //= 10
        if y > 0:
            best = max(best, len(str(y)))
    return best
```

The insertion loop stops early when a prefix is already present, since all its
shorter prefixes were inserted along with it. A trie over digit strings does
the same job and is the natural alternative.

## Complexity

- Time: O((n + m) * D) where `D <= 9` is the number of digits.
- Space: O(n * D) for the set.

## Pitfalls

- Comparing all pairs: O(n * m) is far too slow at `5 * 10^4` each.
- Comparing numbers by value instead of by prefix: `12` and `120` share the
  prefix `"12"`, but `12` and `21` share nothing.
- Treating `0` as a valid prefix. When `y` has been divided down to `0`, no
  digit matched.
