# Hints

## Hint 1
A new request creates a triple booking exactly when it overlaps a stretch of
time that is already booked twice.

## Hint 2
Keep two lists: every accepted booking, and the stretches covered twice. How
do you test a request against the second list?

## Hint 3
Reject the request if it overlaps any double-booked stretch. Otherwise accept
it, and for every accepted booking it meets, add the intersection
`[max(starts), min(ends))` to the double-booked list.
