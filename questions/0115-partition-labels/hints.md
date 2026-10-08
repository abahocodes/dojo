# Hints

## Hint 1
Once a piece contains a letter, it has to stretch at least to that letter's
**last** occurrence in `s`.

## Hint 2
Record the last index of every letter first. Then walk the string, keeping the
furthest last-occurrence of any letter seen in the current piece.

## Hint 3
Track `end = max(end, last[s[i]])`. When `i == end`, nothing in the current
piece appears later, so cut here: record `i - start + 1` and start a new piece
at `i + 1`.
