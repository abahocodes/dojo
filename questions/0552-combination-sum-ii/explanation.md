# Approach: sort, backtrack with a start index, skip repeated picks

Sort the candidates. `backtrack(start, remaining)` tries each card `i >= start`
as the next value of the combination and recurses on `i + 1`, so every card
is used at most once and every combination is built in sorted order.

Duplicates come from choosing *which copy* of a value fills a position. At
one depth, the first copy of a value already explores everything a later
copy could, so skip `candidates[i]` when `i > start` and it equals
`candidates[i - 1]`. Taking a second copy right after the first one
(`i == start`) is still allowed, which is how `[1, 1, 4]` is found.

Sorting also gives pruning: once `candidates[i] > remaining`, every later
card is too big, so the loop can `break`.

```python
def combination_sum2(candidates, target):
    candidates = sorted(candidates)
    result, current = [], []

    def backtrack(start, remaining):
        if remaining == 0:
            result.append(current[:])
            return
        for i in range(start, len(candidates)):
            c = candidates[i]
            if c > remaining:
                break                         # every later card is too big too
            if i > start and c == candidates[i - 1]:
                continue                      # value already tried at this depth
            current.append(c)
            backtrack(i + 1, remaining - c)   # i + 1: each card once
            current.pop()

    backtrack(0, target)
    return result
```

## Complexity

- Time: O(2^n · n) in the worst case, but the `target <= 30` bound and the
  `break` keep the search tree small; with many repeats it is close to the
  size of the output.
- Space: O(target) recursion depth (every card is at least 1), plus the
  output.

## Pitfalls

- Recursing with `i` (as in Combination Sum) reuses the same card.
- Skipping with `i > 0` instead of `i > start` forbids combinations that use
  the same value twice, such as `[1, 1, 4]`.
- Deduplicating at the end with a set is correct but slow on inputs like a
  hundred `1` cards: the search would visit an enormous number of equal
  combinations before discarding them.
- Return `[]` when nothing works, not `[[]]`.
