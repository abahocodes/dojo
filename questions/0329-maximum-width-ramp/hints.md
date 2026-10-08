# Hints

## Hint 1
Checking every pair is O(n^2). Think about which indices can possibly be the
**left** end of the widest ramp.

## Hint 2
If `a < b` and `nums[a] <= nums[b]`, index `b` is never a better left end than
`a`: `a` is further left and at least as easy to satisfy. So the only useful
left ends form a strictly decreasing sequence of prefix minima.

## Hint 3
Push those candidate left ends onto a stack in one forward pass. Then walk `j`
from the right end leftwards: while the value at the stack top is `<= nums[j]`,
pop it and update the answer with `j - top`. Popping is safe because any later
(smaller) `j` would only give that left end a narrower ramp.
