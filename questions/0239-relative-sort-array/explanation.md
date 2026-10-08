# Approach: counting sort

Values are bounded by 1000, so a count array replaces comparisons entirely.
Count every value of `arr1`. Then build the answer in two passes:

1. For each value in `arr2`, in order, append it `count[value]` times and set
   its count to zero, so it is not emitted again.
2. For every value from 0 to 1000 in increasing order, append it as many times
   as its remaining count. Only values absent from `arr2` still have a nonzero
   count, and they come out sorted.

```python
def relative_sort_array(arr1, arr2):
    count = [0] * 1001
    for x in arr1:
        count[x] += 1
    result = []
    for x in arr2:
        result.extend([x] * count[x])
        count[x] = 0
    for x in range(1001):
        result.extend([x] * count[x])
    return result
```

A comparison sort with the key `(position in arr2, or len(arr2) if absent,
value)` also works in `O(n log n)`.

## Complexity

- Time: O(n + m + V), where `V = 1001` is the value range.
- Space: O(V) for the counts, plus the output.

## Pitfalls

- Forgetting to clear the counts of `arr2` values, which emits them a second
  time in the ascending tail.
- Sorting the leftover values by their order of appearance in `arr1`; they
  must be ascending.
- Dropping duplicates: every copy from `arr1` must appear in the output.
