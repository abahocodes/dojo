# Hints

## Hint 1
Values can only go up, so the value that ends up most frequent is one of the
existing elements. Which elements are cheapest to raise to it?

## Hint 2
Sort the array. To make a group equal to `nums[r]`, the cheapest group is a
contiguous run ending at `r`: the elements just below it.

## Hint 3
Raising the window `nums[l..r]` to `nums[r]` costs
`nums[r] * (r - l + 1) - sum(nums[l..r])`. Slide `r` forward, keep the window
sum, and advance `l` while that cost exceeds `k`. The widest window wins.
Watch for overflow in the product.
