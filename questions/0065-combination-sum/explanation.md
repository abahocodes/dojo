# Approach: backtracking with a start index

Build each combination in non-decreasing order of candidate index. The call
`backtrack(start, remaining)` may only add candidates at index `start` or
later, so every multiset is generated in exactly one order and duplicates
never appear. Passing `i` (not `i + 1`) to the recursive call lets the same
candidate be reused.

Sorting first means that once a candidate exceeds the remainder, every later
one does too, so the loop can stop.

```python
def combination_sum(candidates, target):
    candidates = sorted(candidates)
    result, current = [], []

    def backtrack(start, remaining):
        if remaining == 0:
            result.append(current[:])
            return
        for i in range(start, len(candidates)):
            c = candidates[i]
            if c > remaining:
                break               # sorted: every later candidate is too big
            current.append(c)
            backtrack(i, remaining - c)   # i, not i + 1: reuse is allowed
            current.pop()

    backtrack(0, target)
    return result
```

## Complexity

- Time: proportional to the number of nodes in the search tree, bounded by
  roughly O(N^(T/M + 1)) for `N` candidates, target `T` and smallest candidate
  `M`; in practice it is dominated by the size of the output.
- Space: O(T / M) recursion depth, plus the output.

## Pitfalls

- Looping from `0` instead of `start` returns the same combination in several
  orders.
- Recursing with `i + 1` forbids reuse and misses answers like `[2, 2, 2, 2]`.
- Append a **copy** of `current`, not the list itself.
- When no combination exists, return `[]`, not `[[]]`.
