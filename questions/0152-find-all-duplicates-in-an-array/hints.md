# Hints

## Hint 1
A hash set of seen values solves it in O(n) time but O(n) space. What
information about the input could replace the set?

## Hint 2
Values are in `[1, n]`, so each value `v` names a slot of the array: index
`v - 1`. Can you record "I have seen `v`" in that slot without losing the
number stored there?

## Hint 3
Mark a value as seen by making `nums[v - 1]` negative. Use `abs()` when
reading values. If the slot is already negative when you visit `v`, then `v`
is a duplicate.
