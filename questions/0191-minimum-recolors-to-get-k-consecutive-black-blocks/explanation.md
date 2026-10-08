# Approach: count whites in a fixed-size window

Choosing where the final run of `k` black blocks sits fixes the cost: every
white block inside that window must be painted, and nothing else needs to be.
So the answer is the smallest number of `'W'`s in any window of length `k`.

Count the whites in the first window, then slide: the character at `i` enters,
the one at `i - k` leaves.

```python
def minimum_recolors(blocks, k):
    whites = blocks[:k].count("W")
    best = whites
    for i in range(k, len(blocks)):
        whites += (blocks[i] == "W") - (blocks[i - k] == "W")
        best = min(best, whites)
    return best
```

## Complexity

- Time: O(n).
- Space: O(1).

## Pitfalls

- Looking for the longest existing black run and extending it greedily. The
  best window may join several short black runs separated by single whites.
- Forgetting the first window when tracking the minimum (e.g. when
  `k == len(blocks)` there is only one window).
