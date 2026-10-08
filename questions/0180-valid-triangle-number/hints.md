# Hints

## Hint 1
Order does not matter for whether three lengths form a triangle. Sort the
array first. For sorted `a <= b <= c`, which one of the three inequalities is
the only one you still have to check?

## Hint 2
Only `a + b > c` matters. Fix the largest side `c` as the element at index
`k`, then count pairs among indices `0..k-1` whose sum exceeds it.

## Hint 3
Counting those pairs is a two-pointer sweep: start with `i = 0` and
`j = k - 1`. If `a[i] + a[j] > a[k]`, then every `i'` in `i..j-1` also works
with `j`, so add `j - i` and move `j` left; otherwise move `i` right.
