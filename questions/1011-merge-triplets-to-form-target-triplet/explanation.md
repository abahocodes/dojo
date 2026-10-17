# Approach: greedy, merge everything that fits

Merging takes element-wise maximums, so a value in any position can only go
up. That splits the triplets into two kinds:

- A triplet with any value **above** the target in the same position is
  useless: once merged in, that position stays too large. Skip it.
- Every other triplet is safe: merging it can only move each position closer
  to the target, never past it.

Merging all the safe triplets together therefore gives the largest result
that still fits under the target. If that equals the target, the merge
sequence that produced it is the answer; if it falls short in some position,
no safe triplet carries that value and nothing else could supply it.

```python
def merge_triplets(triplets, target):
    best = [0, 0, 0]
    for a, b, c in triplets:
        if a <= target[0] and b <= target[1] and c <= target[2]:
            best = [max(best[0], a), max(best[1], b), max(best[2], c)]
    return best == target
```

Equivalently: return whether, for each position `k`, some safe triplet has
`t[k] == target[k]`.

## Complexity

- Time: O(n), one pass over the triplets.
- Space: O(1).

## Pitfalls

- Merging a triplet just because it matches one target value is wrong if it
  exceeds the target in another position (Example 2).
- You don't need to find an actual sequence of merges; merging every safe
  triplet into one of them is always a valid sequence.
- A single triplet equal to the target needs zero merges, so the answer is
  `True` even when the list has one element.
