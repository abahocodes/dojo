Consider all the different ways to arrange the elements of `nums`, listed in
lexicographic (dictionary) order: compare arrangements element by element,
and the first position where they differ decides which comes first.

Return the arrangement that comes **immediately after** `nums` in that list.
If `nums` is already the last (largest) arrangement, wrap around and return
the first one, i.e. the elements sorted in ascending order. `nums` may
contain repeated values; arrangements that look identical count once.

Try to do it in place with O(1) extra memory.

## Example 1

```
nums   = [2, 4, 3, 1]
output = [3, 1, 2, 4]
```

## Example 2

```
nums   = [5, 5, 2]
output = [2, 5, 5]   # [5, 5, 2] is the largest arrangement
```

## Constraints

- `1 <= len(nums) <= 100`
- `0 <= nums[i] <= 100`
