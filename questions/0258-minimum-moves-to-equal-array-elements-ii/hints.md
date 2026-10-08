# Hints

## Hint 1
The final common value `t` can be chosen freely, and the cost is
`sum(|nums[i] - t|)`. Which `t` minimizes a sum of absolute differences?

## Hint 2
Moving `t` one step right costs +1 for every element at or left of `t` and
saves 1 for every element right of it. The cost stops decreasing at the
median.

## Hint 3
You only need the median, not a full sort: quickselect finds the `n // 2`-th
smallest element in O(n) on average.
