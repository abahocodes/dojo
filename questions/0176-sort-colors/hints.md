# Hints

## Hint 1
Think of the array as three regions growing at the same time: a block of 0s
at the front, a block of 2s at the back, and 1s in between.

## Hint 2
Keep three indices: `low` (where the next 0 goes), `high` (where the next 2
goes) and `mid` (the element being examined). Everything between `mid` and
`high` is still unknown.

## Hint 3
If `nums[mid]` is 0, swap it with `nums[low]` and advance both. If it is 1,
just advance `mid`. If it is 2, swap it with `nums[high]` and decrement
`high`, but do **not** advance `mid`: the value swapped in has not been
examined yet. Stop when `mid > high`.
