# Hints

## Hint 1
All numbers are positive, so extending a subarray never makes its product
smaller. If a window's product is below `k`, so is the product of every
window inside it.

## Hint 2
For each right end, find the smallest left end whose window product is still
below `k`. How many valid subarrays end at `right` then?

## Hint 3
Keep the running product of `nums[left..right]`. After multiplying in
`nums[right]`, divide out `nums[left]` and advance `left` while the product is
at least `k`. Add `right - left + 1` to the answer. Handle `k <= 1` up front:
the answer is 0.
