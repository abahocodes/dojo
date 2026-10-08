# Hints

## Hint 1
Instead of looping over subarrays, turn it around: for each element, count in
how many subarrays it is the minimum. Its contribution is its value times that
count.

## Hint 2
If `left` is the number of choices for the start (positions going left until
a smaller element) and `right` the number of choices for the end (going right
until a smaller element), `arr[i]` is the minimum of `left * right`
subarrays. Both distances come from a monotonic stack.

## Hint 3
With equal values, a subarray like `[2, 2]` would be counted for both 2s. Break
ties asymmetrically: on the left stop at a value strictly smaller, on the
right stop at a value smaller *or equal* (or the other way round). Then every
subarray is attributed to exactly one position: its leftmost (or rightmost)
minimum.
