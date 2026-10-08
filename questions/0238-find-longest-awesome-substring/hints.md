## Hint 1
A multiset of characters can be arranged into a palindrome exactly when at
most one character occurs an odd number of times. Only the parity of each
digit's count matters.

## Hint 2
Ten digits give ten parity bits, so the parities of a prefix fit in a 10-bit
mask. The parities of the substring `s[j:i]` are the XOR of the masks of the
prefixes of length `i` and `j`.

## Hint 3
For each prefix length `i`, the best start is the earliest prefix `j` whose
mask equals `mask_i` (all even) or differs from it in exactly one bit (one odd
digit). Store the first occurrence of every mask in an array of 1024 entries
and check those 11 candidates at each step.
