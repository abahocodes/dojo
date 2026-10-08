# Approach: prefix parity masks

Track the parity of each letter's count in a 10-bit mask (bit `k` is letter
`'a' + k`). If `prefix` is the mask after reading some prefix of `word`, the
substring between two prefixes has parity mask `prefix_j ^ prefix_i`. That
substring is wonderful exactly when the XOR is `0` (all even) or a single bit
(one odd letter).

So, scanning left to right with a table `seen[mask]` of how many earlier
prefixes (including the empty one) had each mask, the number of wonderful
substrings ending at the current position is `seen[cur]` plus
`seen[cur ^ (1 << k)]` for each of the ten letters `k`.

```python
def wonderful_substrings(word):
    seen = [0] * 1024
    seen[0] = 1
    mask = 0
    total = 0
    for ch in word:
        mask ^= 1 << (ord(ch) - ord("a"))
        total += seen[mask]
        for k in range(10):
            total += seen[mask ^ (1 << k)]
        seen[mask] += 1
    return total
```

## Complexity

- Time: O(10 * n): ten lookups per character.
- Space: O(2^10) for the mask counts.

## Pitfalls

- Forgetting `seen[0] = 1` for the empty prefix, which drops every wonderful
  substring that starts at index 0.
- Counting only all-even substrings (`seen[mask]`) and missing the single-odd
  case, or only the single-odd case.
- Overflow: a word of 10^5 identical letters has about 5 * 10^9 wonderful
  substrings. Use a 64-bit total.
