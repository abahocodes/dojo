# Approach: backtracking over palindromic prefixes

First precompute `pal[i][j]`, which is true when `s[i..j]` is a palindrome.
Filling `i` from right to left makes `pal[i + 1][j - 1]` available when it is
needed. Then backtrack: from `start`, every `end` with `pal[start][end]` gives a
valid next piece. Recurse on the rest; reaching the end of the string means
the pieces chosen so far form one complete partition.

```python
def partition(s):
    n = len(s)
    pal = [[False] * n for _ in range(n)]
    for i in range(n - 1, -1, -1):
        for j in range(i, n):
            if s[i] == s[j] and (j - i < 2 or pal[i + 1][j - 1]):
                pal[i][j] = True

    result, current = [], []

    def backtrack(start):
        if start == n:
            result.append(current[:])     # copy! current keeps changing
            return
        for end in range(start, n):
            if pal[start][end]:
                current.append(s[start:end + 1])
                backtrack(end + 1)
                current.pop()

    backtrack(0)
    return result
```

## Complexity

- Time: O(n · 2^n). There are `n − 1` places to cut, so at most `2^(n−1)`
  partitions, and copying each one costs O(n). The table costs O(n²).
- Space: O(n²) for the table and O(n) for the recursion, plus the output.

## Pitfalls

- Appending `current` itself instead of a copy leaves every entry pointing at
  the same list, which is empty at the end.
- Every single character is a palindrome, so the all-single-letters partition
  is always part of the answer.
- The order of pieces inside a partition matters (they must spell `s`); only
  the order of the partitions is free.
