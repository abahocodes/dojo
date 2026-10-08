# Approach: count the most frequent label

Let `m` be the highest count of any label, and `top` the number of labels that
reach it. Think of the schedule as rows of width `n + 1`:

```
A B C        <- row 1
A B _        <- row 2   ... m - 1 full rows
A B          <- final row: one job per label with count m
```

Each of the first `m - 1` rows starts with a most-frequent job, and the
cool-down forces the next one to wait a full row. Other jobs fill the empty
slots, so the time is at least `(m - 1) * (n + 1) + top`.

If there are more jobs than slots, widen the rows: the jobs still never clash,
nothing idles, and the answer is `len(tasks)`. Since every job needs a unit,
the answer is never below `len(tasks)` either.

```python
from collections import Counter

def least_interval(tasks, n):
    counts = Counter(tasks).values()
    m = max(counts)
    top = sum(1 for c in counts if c == m)
    return max(len(tasks), (m - 1) * (n + 1) + top)
```

**Alternative (simulation):** keep a max-heap of remaining counts and a queue
of labels cooling down. Each time unit, run the label with the most jobs left
that is ready, then re-admit it `n + 1` units later. Correct but O(total time ·
log 26), and fiddly to get right.

## Complexity

- Time: O(len(tasks)) to count.
- Space: O(1): at most 26 labels.

## Pitfalls

- Don't forget the `max` with `len(tasks)`: with many distinct labels and a
  small `n`, the formula alone undercounts (e.g. `["A","B","C","D"]`, `n = 1`
  gives `2` from the formula but `4` jobs need 4 units).
- Several labels can share the top count. Each adds one unit to the final row,
  not just one in total.
- There is no trailing idle time: the last row is only as wide as `top`.
