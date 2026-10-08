# Hints

## Hint 1
A letter that appears fewer than `k` times in the whole string can never be
part of a balanced substring. What does that say about where the answer can
lie?

## Hint 2
One route: split the string at every such letter and solve each piece on its
own (divide and conquer). Another: a plain sliding window fails because
"balanced" is not monotone as the window grows. Can you add a constraint that
makes it monotone?

## Hint 3
Fix `t`, the number of distinct letters the window may contain, for each
`t` from 1 to 26. With that cap, grow the right edge, shrink the left while
the window has more than `t` distinct letters, and track how many letters in
the window already reach `k`. When that number equals the distinct count, the
window is balanced.
