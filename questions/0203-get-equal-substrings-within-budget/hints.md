# Hints

## Hint 1
Compute the cost of every position on its own: `cost[i] = |s[i] - t[i]|`. Now
the strings no longer matter.

## Hint 2
You want the longest contiguous block of `cost` whose sum is at most
`max_cost`. All costs are non-negative, so shrinking a block never raises its
sum.

## Hint 3
Slide a window: add `cost[right]`, and while the sum exceeds `max_cost`
subtract `cost[left]` and advance `left`. The best `right - left + 1` is the
answer.
