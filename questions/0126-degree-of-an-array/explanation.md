# Approach: first index, count and span per value

Let `d` be the degree. A subarray with degree `d` contains `d` copies of some
value `x`, and since `x` appears at most `d` times in all of `nums`, it
contains **every** copy of `x`. The shortest such subarray for `x` runs from
its first occurrence to its last. So the answer is the minimum
`last[x] - first[x] + 1` over the values `x` with `count[x] == d`.

That can be done in a single pass: when index `i` holds `x`, the span of `x`
so far is `i - first[x] + 1`. Track the best (count, span) pair: a higher
count wins, and for an equal count the shorter span wins.

```python
def find_shortest_sub_array(nums):
    first = {}
    count = {}
    degree = 0
    best = 0
    for i, x in enumerate(nums):
        first.setdefault(x, i)
        count[x] = count.get(x, 0) + 1
        span = i - first[x] + 1
        if count[x] > degree or (count[x] == degree and span < best):
            degree = count[x]
            best = span
    return best
```

## Complexity

- Time: O(n) expected.
- Space: O(m) for the maps, where `m` is the number of distinct values.

## Pitfalls

- Picking just one value with maximal frequency. When several values tie,
  their spans differ, and you need the smallest.
- Returning the degree instead of the subarray length.
- An array of distinct values has degree 1; any single element works, so
  the answer is 1.
