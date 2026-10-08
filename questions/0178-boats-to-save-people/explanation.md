# Approach: greedy pairing after sorting

Sort the weights. Look at the heaviest remaining person `j` and the lightest
remaining person `i`.

- If `people[i] + people[j] <= limit`, put them together. Exchange argument:
  in any optimal plan, swapping the lightest person into the heaviest
  person's boat keeps every boat within the limit, so some optimal plan pairs
  them.
- Otherwise nobody can share with `j` (everyone else is at least as heavy as
  `i`), so `j` gets a boat alone.

Either way one boat is used and `j` moves left; `i` moves right only when it
was paired.

```python
def num_rescue_boats(people, limit):
    p = sorted(people)
    i, j = 0, len(p) - 1
    boats = 0
    while i <= j:
        if p[i] + p[j] <= limit:
            i += 1
        j -= 1
        boats += 1
    return boats
```

Since weights are at most `3 * 10^4`, a counting sort makes the sort O(n +
limit), but the comparison sort is already fast enough.

## Complexity

- Time: O(n log n) for the sort, O(n) for the sweep.
- Space: O(n) for the sorted copy.

## Pitfalls

- Pairing people greedily in input order, or pairing the two lightest
  people first: `[1, 2, 2, 3]` with `limit = 4` needs 2 boats (`1+3`, `2+2`),
  but pairing `1+2` first forces 3.
- Putting three light people in one boat: each boat holds at most two.
- Stopping at `i < j` and forgetting the last person when `i == j`.
