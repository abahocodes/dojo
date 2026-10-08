# Hints

## Hint 1
Without the circle this is the straight-street problem: at each house, either
skip it or rob it and skip its neighbour. What does the circle actually forbid?

## Hint 2
The only new rule is that house `0` and the last house can't both be robbed.
So at least one of them is skipped in any valid plan.

## Hint 3
Solve the straight-street problem twice, once on `nums[1:]` (first house
skipped) and once on `nums[:-1]` (last house skipped), and return the larger
result. Handle a single house separately.
