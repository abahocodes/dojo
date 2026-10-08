# Hints

## Hint 1
Each letter only cares about its *net* shift: the number of forward
operations covering it minus the number of backward ones. Its final letter is
the original moved by that net amount, modulo 26.

## Hint 2
Computing the net shift of every index by looping over each operation's range
is O(len(s) * len(shifts)). Record each range operation in O(1) instead.

## Hint 3
Use a difference array of length `len(s) + 1`: add `+1` or `-1` at `start`
and the opposite at `end + 1`. A running sum gives each index's net shift;
reduce it into `0..25` with a non-negative modulo before adding it to the
letter.
