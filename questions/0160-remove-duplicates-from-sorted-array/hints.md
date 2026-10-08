# Hints

## Hint 1
Because the array is sorted, equal values sit next to each other. A value
is new exactly when it differs from the one before it.

## Hint 2
Use two indices: `read` visits every element, `write` counts how many
distinct values have been kept so far.

## Hint 3
Start with `write = 1` (the first element is always kept). For each `read`
from 1 onward, if `nums[read] != nums[write - 1]`, copy it to
`nums[write]` and increment `write`. Return `nums[:write]`.
