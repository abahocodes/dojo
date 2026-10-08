# Approach: greedy from the smallest card

The smallest remaining card has nothing below it, so it must start a run of
`group_size` consecutive values. That run is forced, so we can remove it
without losing any valid split, and repeat. To do this efficiently we count the
cards and process distinct values in sorted order. If value `x` still has `c`
copies when we reach it, those `c` copies all start runs at `x`, so the next
`group_size - 1` values each need at least `c` copies too.

```python
from collections import Counter

def is_n_straight_hand(hand, group_size):
    if len(hand) % group_size:
        return False
    count = Counter(hand)
    for x in sorted(count):
        c = count[x]
        if c == 0:
            continue
        for v in range(x, x + group_size):
            if count[v] < c:
                return False
            count[v] -= c
    return True
```

## Complexity

- Time: O(n log n) to sort the distinct values. The inner loop costs
  `group_size` per run start, and there are at most `n / group_size` run
  starts, so it adds only O(n).
- Space: O(n) for the counts.

## Pitfalls

- Starting groups at an arbitrary card (not the smallest) can pick a run that
  blocks a valid split.
- Removing one run at a time, rescanning for the minimum each time, is O(n²).
  Subtract all `c` copies at once.
- In Python, reading `count[v]` for a missing value creates it with `0`; that
  is fine with `Counter`, but iterate over a sorted snapshot of the keys, not
  the live dictionary.
- In JavaScript, sort numbers with a comparator: the default sort is by string.
