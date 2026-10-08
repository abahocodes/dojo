# Approach: expand around every center

A palindrome is determined by its center and its radius. Odd-length palindromes
are centered on a character, even-length ones on the gap between two equal
neighbours. For each of the `2n - 1` centers, expand outwards while the two
ends match; the last matching window is the longest palindrome at that center.

```python
def longest_palindrome(s):
    best_lo, best_len = 0, 1
    for center in range(len(s)):
        for lo, hi in ((center, center), (center, center + 1)):
            while lo >= 0 and hi < len(s) and s[lo] == s[hi]:
                lo -= 1
                hi += 1
            length = hi - lo - 1
            if length > best_len:
                best_lo, best_len = lo + 1, length
    return s[best_lo:best_lo + best_len]
```

**Ties.** Two palindromes of the same length that start at `i < j` have
centers in the same order, and odd and even palindromes never have the same
length. So scanning centers left to right and updating only on a strictly
longer palindrome returns the earliest-starting one.

**DP alternative:** `pal[i][j] = s[i] == s[j] and pal[i+1][j-1]`, filled by
increasing length. Same O(n²) time, but O(n²) memory. Manacher's algorithm
gets O(n) time, though it's rarely expected in an interview.

## Complexity

- Time: O(n²) in the worst case (e.g. `"aaaa...a"`), often much faster.
- Space: O(1) besides the output.

## Pitfalls

- Forgetting even-length centers misses answers like `"abba"`.
- Updating on `>=` instead of `>` returns a later palindrome on ties.
- After the loop exits, `lo` and `hi` point one past the palindrome on each
  side: the substring is `s[lo + 1 : hi]`, of length `hi - lo - 1`.
- A single character is always a palindrome, so the answer is never empty.
