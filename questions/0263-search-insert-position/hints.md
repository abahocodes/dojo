# Hints

## Hint 1
Both cases ask for the same thing: the first index whose value is
**at least** `target`.

## Hint 2
That index splits the array into a prefix of values `< target` and a suffix
of values `>= target`. Binary search for the boundary.

## Hint 3
Use a half-open window `lo = 0`, `hi = len(nums)`. While `lo < hi`, look at
`mid`: if `nums[mid] < target` the boundary is to the right (`lo = mid + 1`),
otherwise it is at `mid` or to its left (`hi = mid`). Return `lo`.
