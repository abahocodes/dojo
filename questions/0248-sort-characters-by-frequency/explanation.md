# Approach: count, sort the distinct characters, expand

The output is fully determined by each character's count and the ordering
rule. Count occurrences in one pass, sort the (at most 62) distinct characters
by `(-count, character)`, then emit each character `count` times.

```python
from collections import Counter


def frequency_sort(s):
    counts = Counter(s)
    order = sorted(counts, key=lambda c: (-counts[c], c))
    return "".join(c * counts[c] for c in order)
```

Bucket sort is an alternative: put each character in bucket `count`, then
walk the buckets from `len(s)` down to `1`, emitting each bucket's characters
in ascending order. With an alphabet this small, sorting the distinct
characters is just as fast and simpler.

## Complexity

- Time: O(n + k log k), where `k <= 62` is the number of distinct
  characters, so O(n) overall.
- Space: O(n) for the output, O(k) for the counts.

## Pitfalls

- Sorting the whole string with a comparator that looks up counts: correct but
  O(n log n) with a heavy constant.
- Ignoring the tie rule: with `"zzY99Y"`, any order of the three groups is a
  valid frequency sort, but only `"99YYzz"` is accepted.
- Treating `'a'` and `'A'` as the same character.
