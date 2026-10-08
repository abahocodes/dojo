# Approach: sort, then check adjacent triples

With sides `a <= b <= c`, the inequalities `a + c > b` and `b + c > a` always
hold, so a triangle has positive area exactly when `a + b > c`.

Fix the longest side `c`. The best partners for it are the two largest values
below it: they maximise `a + b`, which both makes `a + b > c` most likely and
gives the largest perimeter. If even they fail, no triangle has `c` as its
longest side. So after sorting in descending order, we only need to test each
window of three neighbours, from the largest down. The first window that
passes has the largest possible longest side and the best partners for it, so
its perimeter is the answer.

```python
def largest_perimeter(nums):
    nums = sorted(nums, reverse=True)
    for i in range(len(nums) - 2):
        if nums[i] < nums[i + 1] + nums[i + 2]:
            return nums[i] + nums[i + 1] + nums[i + 2]
    return 0
```

## Complexity

- Time: O(n log n) for the sort; the scan is O(n).
- Space: O(n) for the sorted copy (O(1) extra when sorting in place).

## Pitfalls

- Using `>=` instead of `>`: sides `1, 1, 2` are collinear and have zero
  area, so they do not count.
- Trying all O(n^3) triples: far too slow for `10^4` values.
- Returning the first valid triangle found in an unsorted scan; it need not
  be the largest.
