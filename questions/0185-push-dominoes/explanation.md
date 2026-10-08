# Approach: resolve each gap between pushed dominoes

Upright dominoes only ever get pushed by the nearest pushed domino on each
side, so every maximal run of `'.'` can be resolved by looking at the letters
that bound it. Add a virtual `'L'` at index `-1` and a virtual `'R'` at index
`n`: they never push anything into the row, which is exactly how the edges
behave.

For neighbouring letters `x` at `i` and `y` at `j`:

| bounds    | dots between become                                        |
|-----------|------------------------------------------------------------|
| `L ... L` | all `L` (the right one sweeps left)                        |
| `R ... R` | all `R`                                                     |
| `L ... R` | unchanged (each pushes away from the gap)                   |
| `R ... L` | `R` from the left and `L` from the right, meeting in the middle; an odd middle domino stays `.` |

```python
def push_dominoes(dominoes):
    n = len(dominoes)
    res = list(dominoes)
    marks = [(-1, "L")] + [(i, c) for i, c in enumerate(dominoes) if c != "."] + [(n, "R")]
    for (i, x), (j, y) in zip(marks, marks[1:]):
        if x == y:
            for k in range(i + 1, j):
                res[k] = x
        elif x == "R" and y == "L":
            lo, hi = i + 1, j - 1
            while lo < hi:
                res[lo] = "R"
                res[hi] = "L"
                lo += 1
                hi -= 1
    return "".join(res)
```

An equivalent technique computes, for every position, the distance to the
nearest `'R'` on its left with no `'L'` in between and the distance to the
nearest `'L'` on its right with no `'R'` in between; the closer one wins and a
tie stays upright.

## Complexity

- Time: O(n): each position is written at most once.
- Space: O(n) for the result (and the list of pushed positions).

## Pitfalls

- Making the middle domino of `R.L` fall. It is hit from both sides in the
  same second and stays upright.
- Letting a domino be pushed through an already-fallen one: in `R.L.`, the
  last `'.'` is never reached by the `'R'`.
- Forgetting the edges: dots before the first letter fall only if that letter
  is `'L'`, and dots after the last letter only if it is `'R'`.
- Simulating second by second on the full input: a single `R` followed by
  10^5 dots needs 10^5 rounds.
