# Hints

## Hint 1
If you can make every city reach power `x`, you can also make every city
reach any smaller value. That monotonicity means you can binary search on the
answer instead of searching over placements.

## Hint 2
To test a target `x`, sweep the cities left to right. The leftmost city that
is still below `x` must be fixed by new stations whose range covers it. Where
should they go so they help as many later cities as possible?

## Hint 3
Put the missing stations at city `min(n - 1, i + r)`: they still cover city
`i` and reach as far right as possible, up to city `i + 2r`. Track the extra
power with a difference array (add when you build, subtract when it drops out
of range) so each check is O(n). Compute the starting powers with a sliding
window or prefix sums.
