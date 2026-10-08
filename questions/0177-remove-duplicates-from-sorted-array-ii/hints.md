# Hints

## Hint 1
Because the array is sorted, all copies of a value sit next to each other.
You only need to know how many copies of the current value you have already
kept.

## Hint 2
Use a write index `k`: the first `k` slots hold the answer built so far.
Read the elements one by one and decide whether each is written at `k`.

## Hint 3
An element `x` may be written when `k < 2` or `x != nums[k - 2]` (the
element two places back in the output). If `x` equals that element, the
output already ends with two copies of `x`.
