# Hints

## Hint 1
Every good subarray contains `k`, so you can picture growing a window outward
from `[k, k]` one element at a time, either to the left or to the right.

## Hint 2
While the window grows, its minimum can only stay the same or drop. To keep the
minimum as high as possible for as long as possible, which neighbour should you
absorb next?

## Hint 3
Start with `i = j = k`. At each step, extend toward the larger of `nums[i-1]`
and `nums[j+1]` (or toward the only side left), update the running minimum, and
record `minimum * (j - i + 1)`. After `n - 1` steps you have seen the best score.
