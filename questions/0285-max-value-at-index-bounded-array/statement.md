You must build an array `nums` of length `n` that satisfies all of the
following:

- every element is a **positive** integer (at least `1`);
- any two neighbouring elements differ by **at most 1**, i.e.
  `abs(nums[i] - nums[i + 1]) <= 1`;
- the sum of all elements is **at most** `max_sum`.

Among all such arrays, return the largest value that `nums[index]` can take.

## Example 1

```
n       = 5
index   = 1
max_sum = 10
output  = 3    # [2, 3, 2, 1, 1] sums to 9; a 4 at index 1 needs at least 4+3+3+2+1 = 13
```

## Example 2

```
n       = 6
index   = 5
max_sum = 20
output  = 5    # [1, 1, 2, 3, 4, 5] sums to 16; a 6 would need 6+5+4+3+2+1 = 21
```

## Constraints

- `1 <= n <= max_sum <= 10^9`
- `0 <= index < n`
