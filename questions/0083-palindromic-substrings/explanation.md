# Approach: expand around every center

Every palindrome has a center: a single character when its length is odd, or
the gap between two characters when it is even. Number the `2n − 1` centers so
that center `k` starts with `left = k // 2` and `right = left + k % 2`. From
each center, expand outward while `s[left] == s[right]`; each successful step
is one more palindromic substring, and the first mismatch ends all larger
ones with that center.

```python
def count_substrings(s):
    n = len(s)
    count = 0
    for center in range(2 * n - 1):
        left = center // 2
        right = left + center % 2
        while left >= 0 and right < n and s[left] == s[right]:
            count += 1
            left -= 1
            right += 1
    return count
```

**Alternative (dynamic programming):** `dp[i][j]` is true when `s[i..j]` is a
palindrome, which holds when `s[i] == s[j]` and `dp[i+1][j-1]` is true (or the
substring has length at most 2). Count the true cells. Same O(n²) time, but
O(n²) space. Manacher's algorithm brings the time down to O(n).

## Complexity

- Time: O(n²): `2n − 1` centers, each expanding at most `n / 2` times.
- Space: O(1).

## Pitfalls

- Forgetting the even-length centers misses palindromes like `"aa"` and
  `"abba"`.
- Counting distinct palindromes instead of positions: `"aaa"` has 6, not 3.
- With all-equal characters the answer is `n(n + 1) / 2`, so an O(n³)
  approach is noticeably slow at the upper limit.
