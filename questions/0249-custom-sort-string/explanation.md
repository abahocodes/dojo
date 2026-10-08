# Approach: counting for the ranked letters, a filter for the rest

The first part of the answer is fixed by the counts: for each letter of
`order` in turn, write it as many times as it occurs in `s`. The second part
is just `s` with the ranked letters removed, which keeps the unranked letters
in their original relative order.

```python
def custom_sort_string(order, s):
    counts = {}
    for c in s:
        counts[c] = counts.get(c, 0) + 1
    ranked = set(order)
    head = "".join(c * counts.get(c, 0) for c in order)
    tail = "".join(c for c in s if c not in ranked)
    return head + tail
```

Equivalently, a **stable** sort of `s` by the key "position in `order`, or 26
if absent" gives the same string, in O(n log n).

## Complexity

- Time: O(len(order) + len(s)).
- Space: O(len(s)) for the output; the counts take O(26).

## Pitfalls

- Sorting the unranked letters alphabetically, or putting them first: they
  go last, in their original order.
- Using an unstable sort with the rank key: unranked letters would lose
  their relative order.
- Dropping letters of `s` that are missing from `order`.
