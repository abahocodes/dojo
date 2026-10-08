# Hints

## Hint 1
Two strings are anagrams exactly when every letter appears the same number of
times in both. What quick check rules many pairs out immediately?

## Hint 2
If the lengths differ, the answer is `false`. Otherwise, sorting both strings
and comparing works in O(n log n). What if you counted letters instead?

## Hint 3
There are only 26 letters. Keep an array of 26 counters: add 1 for each letter
of `s` and subtract 1 for each letter of `t`. The strings are anagrams exactly
when every counter ends at zero.
