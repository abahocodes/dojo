# Hints

## Hint 1
Trying all triples is O(n³). If you fix the first value `a`, what problem is
left for the other two values?

## Hint 2
With `a` fixed, you need two values summing to `-a`. On a **sorted** list,
that is a classic two-pointer search: move the left pointer right when the sum
is too small, the right pointer left when it is too big.

## Hint 3
Sort once. For each index `i`, skip it if `nums[i] == nums[i - 1]`, then run
two pointers on `i + 1 .. n - 1`. After recording a triple, move both pointers
and keep moving the left one past values equal to the one just used. That
avoids duplicates without needing a set.
