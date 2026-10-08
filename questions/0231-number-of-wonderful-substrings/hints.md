# Hints

## Hint 1
Only the *parity* of each letter's count matters, and there are just ten
letters. Ten parities fit in a 10-bit mask.

## Hint 2
Let `mask[i]` be the parity mask of the prefix `word[0..i-1]`. The parity mask
of the substring `word[i..j-1]` is `mask[j] ^ mask[i]`. A substring is
wonderful when that XOR has at most one bit set.

## Hint 3
Scan left to right, counting how often each prefix mask has appeared so far
(start with the empty prefix, mask `0`). At each position, add the count of
the current mask (all-even substrings ending here) plus the counts of the ten
masks that differ from it in exactly one bit.
