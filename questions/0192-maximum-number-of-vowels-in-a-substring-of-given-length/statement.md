Given a lowercase string `s` and an integer `k`, look at every substring of
`s` that has length **exactly** `k`. Return the largest number of vowels that
any of those substrings contains. The vowels are `a`, `e`, `i`, `o` and `u`
(`y` is not a vowel).

## Example 1

```
s      = "programming"
k      = 4
output = 2    # "ogra" and "ammi" each hold two vowels
```

## Example 2

```
s      = "rhythm"
k      = 2
output = 0
```

## Constraints

- `1 <= k <= len(s) <= 10^5`
- `s` consists of lowercase English letters
