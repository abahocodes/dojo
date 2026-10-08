# Hints

## Hint 1
There may be 2.5 * 10^9 products, so don't build them. Instead, for a value
`x`, count how many products are `<= x`, then binary search on `x`.

## Hint 2
Fix `a = nums1[i]`. If `a > 0`, the products `a * nums2[j]` increase with `j`;
if `a < 0`, they decrease; if `a == 0`, they are all 0. In each case the
products `<= x` form a prefix or a suffix of `nums2`.

## Hint 3
Find that prefix or suffix with a binary search over `nums2` for each `a`.
The search range for `x` is bounded by the four "corner" products
`nums1[0|last] * nums2[0|last]`. Return the smallest `x` whose count reaches
`k`.
