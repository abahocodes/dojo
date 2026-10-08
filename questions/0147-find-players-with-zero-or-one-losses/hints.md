# Hints

## Hint 1
For each player you only need two facts: whether they played at all, and
how many matches they lost.

## Hint 2
Record losses per player in a hash map (or an array indexed by player id).
Make sure winners also get an entry, with zero losses, so you know they
played.

## Hint 3
Collect the players with zero losses and with exactly one loss, then sort
both lists. With an array indexed by id, walking the ids in order produces
sorted lists for free.
