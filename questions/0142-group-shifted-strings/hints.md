# Hints

## Hint 1
Strings of different lengths can never be in the same family. Among strings
of the same length, what stays the same when you shift?

## Hint 2
Shifting changes every letter by the same amount, so the gap between each
pair of neighbouring letters, taken modulo 26, does not change.

## Hint 3
Build a key for every string: the sequence of `(s[i] - s[0]) mod 26` (or of
neighbouring differences mod 26). Strings with equal keys are in the same
family, so group them with a hash map from key to list.
