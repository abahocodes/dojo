## Hint 1
Build the sorted line-up and compare it with the original, position by
position.

## Hint 2
Heights are between 1 and 100, so you do not need a comparison sort: count
how many students have each height.

## Hint 3
Walk the original array while keeping a pointer to the smallest height that
still has a remaining count. That height is what belongs at the current
position; compare, then decrement its count.
