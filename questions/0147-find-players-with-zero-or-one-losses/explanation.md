# Approach: count losses per player

Scan the matches once. Every winner is marked as a player (with no extra
loss); every loser gets one more loss. Afterwards, the players with zero
losses go into the first list and those with exactly one loss into the
second. Sorting the ids (or walking an array indexed by id from small to
large) gives the required order.

```python
def find_winners(matches):
    losses = {}
    for winner, loser in matches:
        losses.setdefault(winner, 0)
        losses[loser] = losses.get(loser, 0) + 1
    never = sorted(p for p, c in losses.items() if c == 0)
    once = sorted(p for p, c in losses.items() if c == 1)
    return [never, once]
```

The Java, C++, Go and JavaScript solutions use an array of size
`max id + 1`, with `-1` meaning "never played", and read it in id order, so
no sort is needed.

## Complexity

- Time: O(m log m) with sorting, or O(m + V) with an array over the ids,
  where m is the number of matches and V the largest id.
- Space: O(number of players), or O(V) for the array.

## Pitfalls

- Treating "never lost" as "zero losses in the map" without registering
  winners: a player who only won would then be missing.
- Including players who never played: they also have zero losses.
- Returning the lists in first-appearance order instead of increasing order.
