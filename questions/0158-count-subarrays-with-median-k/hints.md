# Hints

## Hint 1
A subarray can only have median `k` if it contains `k`. Since all values
are distinct, every other element is either greater or smaller than `k`;
the actual values no longer matter.

## Hint 2
Let `g` be the count of elements greater than `k` in the subarray and `s`
the count smaller. Median `k` means `g == s` (odd length) or `g == s + 1`
(even length, `k` is the left middle). Replace each element with `+1`
(greater) or `-1` (smaller) and look at the sum.

## Hint 3
Split the subarray at `k`'s position. Record, in a hash map, how often each
running sum occurs as you extend to the right of `k` (including the empty
extension). Then extend to the left; for each left sum `L`, add the counts
of right sums `-L` and `1 - L`.
