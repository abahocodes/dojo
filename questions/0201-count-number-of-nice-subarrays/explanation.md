# Approach: prefix counts of odd numbers

Let `odds` be the number of odd elements seen so far. A subarray ending at the
current position is nice exactly when the prefix before its start had
`odds - k` odd elements. So keep a table `seen[p]` = number of prefixes
(including the empty one) with exactly `p` odd elements, and add
`seen[odds - k]` at every step. Prefix counts only range from `0` to `n`, so an
array works as the table.

```python
def number_of_nice_subarrays(nums, k):
    seen = [0] * (len(nums) + 1)
    seen[0] = 1
    odds = total = 0
    for x in nums:
        odds += x & 1
        if odds >= k:
            total += seen[odds - k]
        seen[odds] += 1
    return total
```

A sliding-window version also works: `exactly(k) = at_most(k) - at_most(k - 1)`,
where `at_most(m)` counts subarrays with at most `m` odd elements using two
pointers.

## Complexity

- Time: O(n).
- Space: O(n) for the table (O(1) for the sliding-window version).

## Pitfalls

- Forgetting the empty prefix (`seen[0] = 1`), which misses subarrays that
  start at index 0.
- A plain "shrink while too many odds" window counts only one start per end;
  with exactly `k` you must count every even element you can drop on the left.
- Using `x % 2` is fine here since all numbers are positive.
