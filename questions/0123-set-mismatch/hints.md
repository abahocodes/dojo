# Hints

## Hint 1
Finding the duplicate is easy if you remember which values you've seen.

## Hint 2
Once you know the duplicate, the missing number follows from a sum: compare
the sum of `nums` with `1 + 2 + ... + n`.

## Hint 3
`sum(nums) = n(n + 1)/2 - missing + duplicated`, so
`missing = n(n + 1)/2 - (sum(nums) - duplicated)`.
