A row of dominoes stands on a table. The string `dominoes` describes the row
at time 0: `'L'` is a domino that has just been pushed to the left, `'R'` one
pushed to the right, and `'.'` one still standing upright.

Every second, each domino that is falling to the left pushes its left-hand
neighbour, and each domino falling to the right pushes its right-hand
neighbour. An upright domino that gets pushed from one side only starts
falling in that direction. An upright domino pushed from **both** sides in the
same second is balanced and stays upright forever. A domino that is already
falling or fallen never changes direction.

Return a string describing the final state of the row once nothing moves.

## Example 1

```
dominoes = "R...L"
output   = "RR.LL"   # the middle domino is hit from both sides at once
```

## Example 2

```
dominoes = ".L..R."
output   = "LL..RR"  # the two middle dominoes are never pushed
```

## Constraints

- `1 <= len(dominoes) <= 10^5`
- `dominoes[i]` is `'L'`, `'R'` or `'.'`
