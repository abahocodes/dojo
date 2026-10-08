# Approach: one pass with a balance per digit

Walk both strings together. Equal digits are bulls and take no further part.
For a mismatched pair `(s, g)`, keep a balance array `bal` of 10 counters:
a positive `bal[d]` means unmatched copies of `d` from `secret` are waiting,
a negative one means unmatched copies from `guess` are waiting.

- If `bal[s] < 0`, a waiting guess digit matches this secret digit: one cow.
- If `bal[g] > 0`, a waiting secret digit matches this guess digit: one cow.
- Then `bal[s] += 1` and `bal[g] -= 1`.

Each cow consumes one waiting copy on each side, so the total equals
`sum(min(left_secret[d], left_guess[d]))`.

```python
def get_hint(secret, guess):
    bulls = cows = 0
    bal = [0] * 10
    for s, g in zip(secret, guess):
        if s == g:
            bulls += 1
            continue
        a, b = ord(s) - 48, ord(g) - 48
        if bal[a] < 0:
            cows += 1
        if bal[b] > 0:
            cows += 1
        bal[a] += 1
        bal[b] -= 1
    return f"{bulls}A{cows}B"
```

## Complexity

- Time: O(n), one pass.
- Space: O(1), ten counters.

## Pitfalls

- Counting a bull position again as a cow: bulls must be removed from both
  sides before counting cows.
- Counting cows without multiplicity limits: in `"1123"` / `"0111"` the
  guess has two leftover `1`s but the secret only one, so that is one cow.
