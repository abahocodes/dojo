# Hints

## Hint 1
Start by assuming every machine is its own group: that's `n` groups. What does
adding one cable do to that count?

## Hint 2
A cable either joins two different groups into one (the count drops by one) or
connects two machines that were already in the same group (nothing changes).
You need a fast way to ask "are these two already in the same group?".

## Hint 3
Use union-find: each machine points to a parent, and following parents leads to
a group's representative. For each cable, find both representatives; if they
differ, point one at the other and decrement the count. Path compression and
union by size keep every lookup nearly constant time.
