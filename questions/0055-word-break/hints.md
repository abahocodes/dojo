# Hints

## Hint 1
Greedily taking the longest (or shortest) matching word can paint you into a
corner, as in Example 1 with `"sunflow"`. You need to consider every choice,
but without repeating work.

## Hint 2
Whether the suffix starting at position `i` can be segmented depends only on
`i`. Equivalently, define `ok[i]`: can the first `i` characters be segmented?

## Hint 3
`ok[0] = true`, and `ok[i]` is true if some word `w` ends at `i` (that is,
`s[i - len(w):i] == w`) with `ok[i - len(w)]` true. Put the words in a set and
only try the lengths that actually occur in the dictionary.
