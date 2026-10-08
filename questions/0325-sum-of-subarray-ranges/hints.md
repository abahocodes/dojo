# Hints

## Hint 1
The total of `(max - min)` over all subarrays equals (sum of the maxima of all
subarrays) minus (sum of the minima of all subarrays). Solve those two
sub-problems separately.

## Hint 2
Instead of enumerating subarrays, ask for each index `i`: in how many subarrays
is `nums[i]` the maximum? If the nearest strictly larger element to the left is
at `L` and the nearest larger-or-equal element to the right is at `R`, the
answer is `(i - L) * (R - i)`. The asymmetric tie rule makes sure each subarray
is credited to exactly one index.

## Hint 3
A monotonic stack finds all those boundaries in one pass: keep indices with
decreasing values; when `nums[i]` forces an index `j` off the stack, `i` is
`j`'s right boundary and the new stack top is its left boundary. Add
`nums[j] * (j - left) * (i - j)`. Do the same with an increasing stack for the
minima and subtract.
