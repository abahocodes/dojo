# Hints

## Hint 1
There can be about 5 * 10^7 pairs, too many to list. But the answer is an
integer between 0 and `max(nums) - min(nums)`. Can you count how many pairs
have gap at most `d`?

## Hint 2
After sorting, the pairs with gap at most `d` that end at position `right` are
exactly those whose left end is in a window `[left, right)` where
`nums[right] - nums[left] <= d`. As `right` moves right, `left` never moves back.

## Hint 3
Counting with two pointers costs O(n). Binary search for the smallest `d`
whose count reaches `k`.
