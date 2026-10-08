# Hints

## Hint 1
To get the next arrangement, change as little as possible at the front.
Look at the longest suffix that is in non-increasing order: it is already
the largest it can be, so something before it has to grow.

## Hint 2
Let `i` be the index just before that suffix. Replace `nums[i]` with the
smallest value in the suffix that is strictly greater than it. If there is
no such `i`, the whole array is non-increasing: return it sorted.

## Hint 3
After swapping `nums[i]` with that value (the rightmost one greater than
`nums[i]`), the suffix is still non-increasing. Reverse it to make it the
smallest possible order.
