# Hints

## Hint 1
For an array of length `n`, the answer is always somewhere in `1..n+1`.
Values outside `1..n` can never affect it.

## Hint 2
A hash set of the values gives O(n) time but O(n) memory. To stay in O(1)
extra space, use the array itself as the "set": index `i` can record
whether the value `i + 1` is present.

## Hint 3
Cyclic sort: while `nums[i]` is in `1..n` and `nums[nums[i] - 1] != nums[i]`,
swap it into its home slot `nums[i] - 1`. Afterwards the first index `i`
with `nums[i] != i + 1` gives the answer `i + 1`; if there is none, it is
`n + 1`.
