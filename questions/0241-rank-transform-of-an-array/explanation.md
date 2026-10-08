# Approach: sort the distinct values, then map each element

The rank of `x` is one plus the number of distinct values below `x`. Sorting
the distinct values puts them in rank order, so walking that list assigns
ranks `1, 2, 3, ...`. A dictionary from value to rank then translates the
original array in one pass, keeping the original positions.

```python
def array_rank_transform(arr):
    rank = {}
    for x in sorted(set(arr)):
        rank[x] = len(rank) + 1
    return [rank[x] for x in arr]
```

The C++ solution skips the map: it deduplicates the sorted copy and finds
each element's rank with `lower_bound`.

## Complexity

- Time: O(n log n) for sorting.
- Space: O(n) for the sorted copy and the map.

## Pitfalls

- Using the index in the sorted array *with* duplicates as the rank. Equal
  values would get different ranks and later ranks would have gaps.
- Sorting the input in place and losing the original order.
- Default string sort in JavaScript: `[10, 9].sort()` gives `[10, 9]`. Pass a
  numeric comparator.
- The empty array is valid input; return an empty array.
