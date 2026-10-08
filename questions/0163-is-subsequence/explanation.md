# Approach: greedy two pointers

Scan `t` once, keeping a pointer `i` to the next character of `s` that
still needs a match. Whenever the current character of `t` equals `s[i]`,
match it and advance `i`. Matching greedily at the earliest opportunity is
safe: any valid matching can be shifted to use the earliest occurrence,
which only leaves more of `t` for the rest of `s`.

```python
def is_subsequence(s, t):
    i = 0
    for ch in t:
        if i < len(s) and s[i] == ch:
            i += 1
    return i == len(s)
```

## Complexity

- Time: O(len(t)).
- Space: O(1).

## Pitfalls

- Reading `s[i]` after `i` has reached `len(s)`: check the bound first.
- Forgetting that an empty `s` is always a subsequence, even of an empty
  `t`.
- Confusing subsequence with substring: the matched characters need not be
  adjacent.
