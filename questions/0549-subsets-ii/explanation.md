# Approach: sort, then backtrack and skip repeated picks

Sort `nums` so equal values are adjacent. Then build subsets the same way as
in plain Subsets: `backtrack(start)` records the current subset, then tries
each `nums[i]` with `i >= start` as the next value and recurses on `i + 1`.

Duplicates come from choosing *which copy* of a value fills a position. At a
given depth, the first copy of a value already leads to every subset that a
later copy could lead to (the later copy just has fewer options after it).
So in the loop, skip `nums[i]` if it equals `nums[i - 1]` and `i > start`.
The condition `i > start` matters: taking the second copy right after the
first one (`i == start`) is how `[2, 2]` gets built, and must stay allowed.

```python
def subsets_with_dup(nums):
    nums = sorted(nums)
    result, current = [], []

    def backtrack(start):
        result.append(current[:])
        for i in range(start, len(nums)):
            if i > start and nums[i] == nums[i - 1]:
                continue                  # this value was already tried here
            current.append(nums[i])
            backtrack(i + 1)
            current.pop()

    backtrack(0)
    return result
```

**Alternative (counts):** group the sorted values into `(value, count)`
pairs and, for each distinct value, branch on taking it `0, 1, ..., count`
times. This builds each multiset exactly once with no skip rule.

## Complexity

- Time: O(n · 2^n) in the worst case (all values distinct), since each of up
  to `2^n` subsets is copied in O(n). Repeats shrink the output.
- Space: O(n) recursion depth and current subset, plus the output.

## Pitfalls

- Forgetting to sort: the skip rule only works when equal values are
  neighbours.
- Writing `i > 0` instead of `i > start` loses subsets such as `[2, 2]`,
  because the second copy is then never picked right after the first.
- Deduplicating with a set of tuples works but is slower and still needs each
  subset sorted to make equal subsets look alike.
- Append a **copy** of `current`, not the list itself.
