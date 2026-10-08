# Hints

## Hint 1
Walk both strings together. Each position pairs a character of `s` with the
character of `t` it must turn into.

## Hint 2
Remember what every character of `s` has been mapped to. A pair that
contradicts an earlier pair means the answer is `false`. Is checking that one
direction enough?

## Hint 3
No: `"ab"` and `"cc"` never contradict the `s -> t` map, yet two letters land
on `c`. Keep a second map from `t` back to `s` and check both on every pair.
