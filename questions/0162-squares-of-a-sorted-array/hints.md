# Hints

## Hint 1
Squaring reverses the order of the negative part. Where in `nums` can the
largest square come from?

## Hint 2
The largest square is at one of the two ends of `nums`: either the most
negative or the most positive element.

## Hint 3
Keep pointers `lo = 0` and `hi = n - 1`, and fill the output from the back.
Compare `abs(nums[lo])` with `abs(nums[hi])`, write the larger square at
the current output slot, and move that pointer inward.
