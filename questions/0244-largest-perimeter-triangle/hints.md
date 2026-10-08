# Hints

## Hint 1
For three sides `a <= b <= c`, only one of the three triangle inequalities can
fail. Which one?

## Hint 2
Sort the lengths. If you decide that `c` is the longest side, which two other
sides give it the best chance of passing `a + b > c`, and also the biggest
perimeter?

## Hint 3
Sort in descending order and look at consecutive windows
`nums[i], nums[i+1], nums[i+2]`. The first window with
`nums[i] < nums[i+1] + nums[i+2]` is the answer; if none passes, return `0`.
