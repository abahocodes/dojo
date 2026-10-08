# Approach: count, then scan in order

A string is distinct only if it appears once in the **entire** array, so
count everything first with a hash map. Then walk `arr` from the start;
each string with count 1 is the next distinct string. Stop at the `k`-th.

```python
from collections import Counter


def kth_distinct(arr, k):
    count = Counter(arr)
    for s in arr:
        if count[s] == 1:
            k -= 1
            if k == 0:
                return s
    return ""
```

## Complexity

- Time: O(n * L) for `n` strings of length up to `L`.
- Space: O(n * L) for the counts.

## Pitfalls

- Iterating over the hash map instead of the array: map order is not
  appearance order in every language.
- Treating "distinct" as "first occurrence of each value". A string that
  repeats is never distinct, not even its first copy.
- Off-by-one on `k`: it is 1-based.
