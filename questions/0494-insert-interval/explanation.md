# Approach: left part, merge zone, right part

Because `intervals` is sorted and disjoint, the intervals that meet
`new_interval = [start, end]` form one contiguous run. Walk the list once:

1. While the current interval ends before `start` (`iv[1] < start`), it lies
   wholly to the left. Copy it.
2. While the current interval begins no later than `end` (`iv[0] <= end`), it
   shares at least one point with the growing new interval. Absorb it:
   `start = min(start, iv[0])`, `end = max(end, iv[1])`.
3. Append `[start, end]`, then copy everything that is left: it begins after
   `end`.

Using strict `<` in step 1 and `<=` in step 2 is what makes touching
intervals merge.

```python
def insert_interval(intervals, new_interval):
    result = []
    start, end = new_interval
    i, n = 0, len(intervals)
    while i < n and intervals[i][1] < start:
        result.append(intervals[i])
        i += 1
    while i < n and intervals[i][0] <= end:
        start = min(start, intervals[i][0])
        end = max(end, intervals[i][1])
        i += 1
    result.append([start, end])
    result.extend(intervals[i:])
    return result
```

## Complexity

- Time: O(n), a single pass. Sorting again would cost O(n log n) for nothing.
- Space: O(n) for the output.

## Pitfalls

- Using `<=` in step 1 or `<` in step 2, which leaves touching intervals such
  as `[1, 2]` and `[2, 3]` unmerged.
- Forgetting the empty-list case: the answer is just `[new_interval]`.
- Appending the merged interval at the end instead of in its sorted position
  when no interval meets it.
