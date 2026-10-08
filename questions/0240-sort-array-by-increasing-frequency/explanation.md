# Approach: count, then sort with a two-level key

The order depends on a property of each value (its frequency) that is not
visible from the element alone, so compute it first: one pass fills a
frequency table. Then sort the elements by `(frequency ascending, value
descending)`. Equal values have equal keys, so their copies end up adjacent
automatically.

```python
def frequency_sort_numbers(nums):
    count = {}
    for x in nums:
        count[x] = count.get(x, 0) + 1
    return sorted(nums, key=lambda x: (count[x], -x))
```

Because values are confined to `[-100, 100]`, Java, C++ and Go use an array of
201 counters indexed by `x + 100` instead of a map.

## Complexity

- Time: O(n log n) for the sort; counting is O(n).
- Space: O(n) for the output and the counts.

## Pitfalls

- Breaking ties by ascending value. Equal frequencies go larger value first.
- Sorting the distinct values and forgetting to repeat each one by its count.
- Negative values: an array-based counter needs an offset (`x + 100`), or
  negative indices go out of bounds (or, in Python, silently wrap around).
