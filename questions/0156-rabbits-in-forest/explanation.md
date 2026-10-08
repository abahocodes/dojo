# Approach: count answers, then pack greedily

If a rabbit says `x`, its color group has exactly `x + 1` members. Two
rabbits with different answers therefore have different colors, so each
distinct answer can be treated on its own.

Suppose `c` interviewed rabbits all answered `x`. One color group can hold at
most `x + 1` of them, so at least `ceil(c / (x + 1))` groups are needed, and
that many groups is also achievable by filling them greedily. Each group
contributes all `x + 1` rabbits, interviewed or not.

```python
def num_rabbits(answers):
    counts = {}
    for x in answers:
        counts[x] = counts.get(x, 0) + 1
    total = 0
    for x, c in counts.items():
        size = x + 1
        groups = (c + size - 1) // size
        total += groups * size
    return total
```

Because answers are below 1000, the other solutions use a fixed array of 1000
counters instead of a hash map.

## Complexity

- Time: O(n) for counting plus O(number of distinct answers).
- Space: O(number of distinct answers), at most 1000.

## Pitfalls

- Adding `x + 1` once per distinct answer: three rabbits saying `1` cannot
  all fit in one group of 2.
- Forgetting answer `0`: such a rabbit is alone, so every one of them counts
  separately.
- Off-by-one in the ceiling: use `(c + size - 1) // size`, not `c // size`.
