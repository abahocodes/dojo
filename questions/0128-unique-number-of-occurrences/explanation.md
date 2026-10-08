# Approach: count, then check the counts for duplicates

Count every value with a hash map. The answer is `true` exactly when the
frequencies are pairwise different, that is, when the set of frequencies is
as large as the number of distinct values.

```python
from collections import Counter


def unique_occurrences(arr):
    freqs = Counter(arr).values()
    return len(set(freqs)) == len(freqs)
```

## Complexity

- Time: O(n) expected.
- Space: O(m) for the counts and the set of frequencies.

## Pitfalls

- Comparing the number of distinct values with `len(arr)`; that tests
  whether values are distinct, not whether their counts are.
- Forgetting negative values when using an array instead of a map for
  counting: shift by 1000 to index from 0.
- A single distinct value always gives `true`.
