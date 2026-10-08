You are given an array `nums` of `n + 1` integers, each between `1` and `n`
inclusive. By the pigeonhole principle some value must appear more than once;
here **exactly one value** is repeated (it may appear two or more times), and
every other value appears at most once.

Return the repeated value. Treat `nums` as read-only (do not reorder or mark
it) and use only O(1) extra memory.

## Example 1

```
nums   = [2, 5, 1, 4, 2, 3]
output = 2
```

## Example 2

```
nums   = [4, 4, 4, 1, 4]
output = 4      # the repeated value may occur many times
```

## Constraints

- `1 <= n <= 10^5`, `len(nums) == n + 1`
- `1 <= nums[i] <= n`
- Exactly one value appears more than once.

**Follow-up:** solve it in O(n) time.
