# Hints

## Hint 1
Whatever you remove is a prefix plus a suffix of `nums`. What is left behind?

## Hint 2
The elements you keep form one contiguous block whose sum is
`sum(nums) - x`. Minimising the removed elements means maximising the length
of that block.

## Hint 3
All values are positive, so a sliding window works: grow the right end, and
while the window sum exceeds the target shrink from the left. Record the
longest window whose sum equals the target exactly. Handle a negative target
(impossible) and a zero target (remove everything).
