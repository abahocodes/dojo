# Hints

## Hint 1
Checking every pair works but costs O(n²). For each number, what single value
would you need to have already seen?

## Hint 2
If the current value is `x`, you're looking for `target - x`. Which data
structure answers "have I seen this value, and where?" in O(1)?

## Hint 3
Walk the list once. Before storing `x` in a map of value → index, check whether
`target - x` is already in it. Checking first avoids pairing a number with itself.
