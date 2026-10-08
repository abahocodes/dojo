# Hints

## Hint 1
Start with the one-dimensional version: how many subarrays of an array sum to
`target`? Prefix sums and a hash map of how often each prefix sum has
appeared answer that in O(n).

## Hint 2
Fix a top row and a bottom row. Collapse the rows between them into one
array whose entry `c` is the sum of column `c` over those rows. Every
submatrix spanning exactly those rows is a subarray of that array.

## Hint 3
For each top row, extend the bottom row one at a time, adding the new row
into the running column sums, and run the 1D counting on the result. That is
O(rows^2 * cols) in total.
