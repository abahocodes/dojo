# Approach: prefix parity masks with first occurrences

A string can be rearranged into a palindrome exactly when at most one
character occurs an odd number of times. Only parities matter, and there are
ten digits, so keep a 10-bit mask where bit `d` is the parity of the count of
digit `d` in the prefix `s[:i]`.

The parity vector of a substring `s[j:i]` is `mask_i XOR mask_j`. That
substring is awesome when this XOR is `0` or has exactly one bit set, i.e.
when `mask_j` is `mask_i` or `mask_i ^ (1 << d)` for some digit `d`. To make
the substring as long as possible, `j` should be the **earliest** prefix with
such a mask. So record `first[m]`, the first prefix index where mask `m`
appears (`first[0] = 0` for the empty prefix), and at each `i` try the 11
candidate masks.

```python
def longest_awesome(s):
    n = len(s)
    first = [n + 1] * 1024      # n + 1 = "not seen yet", gives a negative length
    first[0] = 0
    mask = 0
    best = 0
    for i, ch in enumerate(s, 1):
        mask ^= 1 << (ord(ch) - 48)
        best = max(best, i - first[mask])
        for d in range(10):
            best = max(best, i - first[mask ^ (1 << d)])
        if first[mask] > i:
            first[mask] = i
    return best
```

## Complexity

- Time: O(11 * n) = O(n).
- Space: O(1024) = O(1) for the first-occurrence table.

## Pitfalls

- Forgetting the empty prefix (`first[0] = 0`). Without it the whole string
  can never count as awesome.
- Recording the *latest* occurrence of a mask instead of the earliest. Later
  starts only give shorter substrings.
- Checking only `mask_i` itself. That finds substrings where every count is
  even and misses odd-length palindromes with one middle digit.
- Using a sentinel like `-1` for unseen masks, which makes `i - first[m]`
  larger than `i`. Use a value that yields a non-positive length instead.
- Trying every substring: `O(n^2)` is about `5 * 10^9` checks at the limit.
