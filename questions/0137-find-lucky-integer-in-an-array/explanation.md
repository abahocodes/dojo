# Approach: count, then scan from the top

Count each value's frequency. Because values are at most 500, a fixed array
of counters suffices. Then scan candidate values from largest to smallest;
the first `v` whose count equals `v` is the largest lucky value.

```python
def find_lucky(arr):
    count = [0] * 501
    for x in arr:
        count[x] += 1
    for v in range(500, 0, -1):
        if count[v] == v:
            return v
    return -1
```

## Complexity

- Time: O(n + M), with M = 500 the value bound.
- Space: O(M).

## Pitfalls

- Returning the first lucky value found in array order instead of the
  largest.
- A value that occurs more times than itself (three `2`s) is not lucky.
- Scanning from 0: `count[0] == 0` would wrongly report 0 as lucky.
