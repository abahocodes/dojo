# Approach: backtracking over unused values

Fill positions left to right. At each step, any value not yet in the current
arrangement may go next. Try each one, recurse to fill the remaining
positions, then undo the choice so the next value can be tried. When every
position is filled, record a copy.

```python
def permute(nums):
    result, current = [], []
    used = [False] * len(nums)

    def backtrack():
        if len(current) == len(nums):
            result.append(current[:])     # copy! current keeps changing
            return
        for i, x in enumerate(nums):
            if not used[i]:
                used[i] = True
                current.append(x)
                backtrack()
                current.pop()
                used[i] = False

    backtrack()
    return result
```

**Alternative (swapping):** fix position `start` by swapping each of
`nums[start:]` into it, recurse on `start + 1`, and swap back. It avoids the
`used` array.

**Library:** `itertools.permutations(nums)` does exactly this, but an
interviewer wants to see the recursion.

## Complexity

- Time: O(n · n!): `n!` permutations, each copied in O(n).
- Space: O(n) for the recursion and bookkeeping, plus the O(n · n!) output.

## Pitfalls

- Appending `current` itself instead of a copy leaves `n!` references to one
  list that ends up empty.
- Forgetting to reset `used[i]` (or to pop) after the recursive call corrupts
  every later branch.
- Checking `x in current` instead of a `used` array works for distinct values
  but costs O(n) per check.
