# Hints

## Hint 1
Taking every other house (all even or all odd positions) is not always best:
look at Example 1. Think about the decision for a single house instead.

## Hint 2
For house `i` you either skip it, keeping the best total for the first `i`
houses, or rob it, adding `nums[i]` to the best total for the first `i - 1`
houses.

## Hint 3
`best[i] = max(best[i - 1], best[i - 2] + nums[i])`. Each step only needs the
two previous values, so two variables are enough.
