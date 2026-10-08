# Approach: counting sort, compared on the fly

The expected line-up is just `heights` sorted, and the answer is the number of
positions where the two arrays disagree. Sorting with a library call gives an
`O(n log n)` solution. Because heights are limited to `1..100`, a counting
sort is enough: count each height, then walk the original array while a
pointer `expected` moves up through the heights that still have students left.
The value under the pointer is what the sorted array holds at that position.

```python
def height_checker(heights):
    count = [0] * 101
    for h in heights:
        count[h] += 1
    mismatches = 0
    expected = 1
    for h in heights:
        while count[expected] == 0:
            expected += 1
        if h != expected:
            mismatches += 1
        count[expected] -= 1
    return mismatches
```

## Complexity

- Time: O(n + V) with `V = 100` possible heights.
- Space: O(V) for the counts.

## Pitfalls

- Sorting `heights` in place and then comparing it with itself: you need the
  original order as well as the sorted one.
- Counting the students that must *move* by a minimum number of swaps. The
  question asks only for mismatched positions.
- Starting the `expected` pointer at 0 is harmless, but forgetting to skip
  heights whose count is exhausted is not.
