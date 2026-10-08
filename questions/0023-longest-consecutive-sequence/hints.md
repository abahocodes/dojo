# Hints

## Hint 1
Sorting the values and scanning for neighbours that differ by exactly 1 works
in O(n log n). To beat that, what lookup would let you ask "is `x + 1`
present?" in O(1)?

## Hint 2
Put every value in a hash set. From any value `x`, you can count a run by
checking `x + 1`, `x + 2`, ... in the set. But starting from every value can
cost O(n²) on a long run. Which values are worth starting from?

## Hint 3
Only start counting at a value `x` whose predecessor `x - 1` is **not** in the
set: that `x` is the beginning of a run. Each value is then visited by at most
one counting walk, so the whole thing is O(n).
