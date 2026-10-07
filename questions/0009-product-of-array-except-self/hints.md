# Hints

## Hint 1
Computing each product from scratch is O(n²). Split "everything except `i`"
into two parts that you could compute incrementally.

## Hint 2
`out[i]` = (product of everything to the left of `i`) × (product of everything
to the right of `i`). Both can be built in a single pass each.

## Hint 3
First pass left to right: write the running prefix product into `out[i]` before
multiplying in `nums[i]`. Second pass right to left: keep a running suffix
product and multiply it into `out[i]`, again before including `nums[i]`.
