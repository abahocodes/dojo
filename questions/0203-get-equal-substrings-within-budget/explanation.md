# Approach: sliding window over per-position costs

The cost of converting `s[l..r]` is the sum of the independent per-position
costs `|s[i] - t[i]|`. Those costs are non-negative, so a window's sum only
grows as it widens, which is exactly what a two-pointer window needs. Grow the
right edge; whenever the running cost exceeds the budget, shrink the left edge.
The window after each step is the longest affordable one ending at `right`.

```python
def equal_substring(s, t, max_cost):
    left = cost = best = 0
    for right in range(len(s)):
        cost += abs(ord(s[right]) - ord(t[right]))
        while cost > max_cost:
            cost -= abs(ord(s[left]) - ord(t[left]))
            left += 1
        best = max(best, right - left + 1)
    return best
```

## Complexity

- Time: O(n). Each index enters and leaves the window once.
- Space: O(1).

## Pitfalls

- Treating the budget as "number of changed characters" instead of the sum of
  code differences.
- When a single position costs more than `max_cost` the window becomes empty
  (`left = right + 1`); the length formula then gives 0, which is correct.
- Sorting the costs to pick the cheapest ones ignores that the substring must
  be contiguous.
