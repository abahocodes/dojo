# Hints

## Hint 1
After `i + 1` additions the running total is `start` plus the prefix sum
`nums[0] + ... + nums[i]`.

## Hint 2
Every one of those totals must be at least `1`. Only the smallest prefix sum
really matters.

## Hint 3
If the smallest prefix sum is `low`, you need `start + low >= 1`, i.e.
`start >= 1 - low`. Don't forget `start` itself must be at least `1`.
