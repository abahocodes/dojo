An array was supposed to hold every integer from `1` to `n` exactly once (in
some order). Due to a mistake, one value was overwritten with a copy of a
different value from the array. As a result, one number now appears **twice**
and another number is **missing**.

Given the damaged array `nums`, return `[duplicated, missing]`.

## Example 1

```
nums   = [1, 3, 3, 4]
output = [3, 2]
```

## Example 2

```
nums   = [2, 2]
output = [2, 1]
```

## Constraints

- `2 <= n == len(nums) <= 10^4`
- `1 <= nums[i] <= n`
- Exactly one value appears twice and exactly one value in `[1, n]` is
  missing; every other value appears once.
