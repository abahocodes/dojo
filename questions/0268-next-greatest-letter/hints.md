# Hints

## Hint 1
The array is sorted, so every letter greater than `target` sits in one block
at the right end. You want the first element of that block.

## Hint 2
Binary search for the first index whose letter is strictly greater than
`target` (an "upper bound"). Be careful to skip over copies of `target`
itself.

## Hint 3
Keep `lo = 0, hi = n`. While `lo < hi`, look at `mid`: if
`letters[mid] <= target` the answer is to the right (`lo = mid + 1`),
otherwise `hi = mid`. The answer is `letters[lo]`, or `letters[0]` when
`lo == n` (use `lo % n`).
