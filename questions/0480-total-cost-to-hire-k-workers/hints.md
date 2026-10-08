# Hints

## Hint 1
Removing an applicant from the middle of a list is slow, and rescanning
`2 * candidates` people every round is too. What actually changes from one
round to the next?

## Hint 2
Only one applicant leaves per round, and the window that lost someone pulls in
the next applicant from the untouched middle. Keep the front window and the
back window in two separate min-heaps keyed by `(cost, index)`.

## Hint 3
Use two pointers `i` (next from the front) and `j` (next from the back). Each
round, refill both heaps up to `candidates` while `i <= j`, then pop from
whichever heap has the smaller `(cost, index)` top. Once `i > j`, everyone
left is in one of the heaps.
