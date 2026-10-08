# Hints

## Hint 1
Order does not matter for an anagram, only how many times each letter
appears. Count the letters of both strings.

## Hint 2
A character of `t` can be kept if it can be matched to an equal, still
unmatched character of `s`. Every unmatched character of `t` must be
rewritten, and rewriting it can fix exactly one missing letter.

## Hint 3
For each letter, `s` needs `count_s` copies and `t` has `count_t`. The answer
is the sum over all letters of `max(0, count_s - count_t)`.
