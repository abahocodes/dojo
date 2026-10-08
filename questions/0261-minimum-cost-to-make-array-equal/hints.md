# Hints

## Hint 1
Fix the final value `x`. The total cost `sum(cost[i] * |nums[i] - x|)` is a
sum of V-shaped functions of `x`, so it is convex. Where can the minimum of
such a function be? Do you ever need an `x` that is not one of the `nums`?

## Hint 2
Think of `cost[i]` as `cost[i]` copies of the value `nums[i]`. Making all
copies equal at minimum total distance is the classic median problem.

## Hint 3
Sort the pairs by `nums`. Walk through them adding up `cost`; the first value
at which the running sum reaches half of the total cost is a weighted median.
Make everything equal to it and add up the cost.
