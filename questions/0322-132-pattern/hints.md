# Hints

## Hint 1
Fix the middle index `j`. The best choice for `nums[i]` is then the minimum of
everything before `j`. What do you need to know about the elements after `j`?

## Hint 2
Scan from the right instead. Try to keep track of the largest value that can
play the role of `nums[k]` (the "2"), that is, the largest value that already
has a bigger value (a "3") to its left among the scanned elements.

## Hint 3
Keep a decreasing stack of values seen from the right and a variable `third`
(initially -infinity). For each new value `x`: if `x < third`, a pattern is
found. Otherwise pop every stack value smaller than `x`, setting `third` to
the popped value (each pop has `x` as its "3"), then push `x`.
