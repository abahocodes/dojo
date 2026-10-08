# Approach: fixed-size sliding window

Keep the vowel count of the current window of length `k`. Moving the window one
step right adds `s[i]` and removes `s[i - k]`, so the count changes by at most
one and can be updated in O(1).

```python
def max_vowels(s, k):
    vowels = set("aeiou")
    count = sum(c in vowels for c in s[:k])
    best = count
    for i in range(k, len(s)):
        count += (s[i] in vowels) - (s[i - k] in vowels)
        best = max(best, count)
    return best
```

## Complexity

- Time: O(n).
- Space: O(1).

## Pitfalls

- Forgetting to include the first window in the maximum.
- Treating `y` as a vowel.
- Rebuilding each substring (`s[i:i + k]`) inside the loop, which is O(n * k)
  time and creates lots of garbage.
