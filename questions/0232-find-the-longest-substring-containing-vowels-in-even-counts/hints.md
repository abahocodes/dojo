# Hints

## Hint 1
Only whether each vowel's count is odd or even matters. Five vowels give
2^5 = 32 possible parity states, which fit in a 5-bit mask.

## Hint 2
Let `state[i]` be the parity mask of the prefix `s[0..i-1]`. The substring
`s[i..j-1]` has every vowel even exactly when `state[i] == state[j]`.

## Hint 3
So the answer is the largest distance between two equal prefix states. Record
the first index at which each state appears (the empty prefix has state `0`
at index `-1`), and at each position compare against that first occurrence.
