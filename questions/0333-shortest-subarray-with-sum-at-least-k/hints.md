# Hints

## Hint 1
With prefix sums `P[0] = 0`, `P[j+1] = P[j] + nums[j]`, the block `nums[i..j-1]`
sums to `P[j] - P[i]`. You want the closest pair `i < j` with
`P[j] - P[i] >= k`. A plain two-pointer window fails because negative values
make the sums non-monotonic.

## Hint 2
If `i1 < i2` and `P[i1] >= P[i2]`, start `i1` is never useful: `i2` is later
(shorter blocks) and has a smaller prefix (larger totals). So candidate starts
can be kept with strictly increasing prefix sums.

## Hint 3
Keep those candidate starts in a deque. For each new `j`: while
`P[j] - P[front] >= k`, record `j - front` and pop the front (a later `j`
could only give a longer block for it). Then pop from the back while
`P[back] >= P[j]`, and push `j`.
