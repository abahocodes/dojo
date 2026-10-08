# Approach: first occurrence of each parity mask

Give each vowel one bit and keep `mask`, the parities of the vowel counts in
the prefix read so far. The substring between two prefixes has all vowels
even exactly when the two prefixes have the same mask, because XOR-ing equal
masks gives zero.

To make that substring as long as possible, pair the current position with
the *earliest* prefix that had the same mask. Store the first index for each
of the 32 masks, with mask `0` already seen at index `-1` (the empty prefix).

```python
def find_the_longest_substring(s):
    bit = {"a": 1, "e": 2, "i": 4, "o": 8, "u": 16}
    first = [None] * 32
    first[0] = -1
    mask = 0
    best = 0
    for i, ch in enumerate(s):
        mask ^= bit.get(ch, 0)
        if first[mask] is None:
            first[mask] = i
        else:
            best = max(best, i - first[mask])
    return best
```

## Complexity

- Time: O(n), one pass.
- Space: O(1): a table of 32 first indices.

## Pitfalls

- Storing the *latest* index of each mask instead of the first, which
  shortens the substrings found.
- Missing the empty prefix at index `-1`: without it, a prefix of `s` whose
  vowels are all even (possibly all of `s`) is never counted.
- Brute force over all substrings is O(n^2), far too slow for 5 * 10^5.
