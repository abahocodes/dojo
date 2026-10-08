# Hints

## Hint 1
Checking every substring is far too slow. Notice that if a window covers `t`,
any larger window containing it also covers `t`. That monotonicity suggests two
pointers moving in the same direction.

## Hint 2
Grow the window by moving its right edge until it covers `t`. Then shrink it from
the left for as long as it still covers `t`, recording the best window seen.
Repeat until the right edge reaches the end.

## Hint 3
Keep a count of how many of each character you still `need`, plus a single
integer `missing` = how many characters of `t` are still unmatched. Adding a
character with `need[c] > 0` decreases `missing`; the window covers `t` exactly
when `missing == 0`. Only replace the best window on a strictly shorter length
so the leftmost one wins ties.
