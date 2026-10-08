You are given a list of strings `words` and a string `s`. Count how many
entries of `words` are a prefix of `s`: the word equals the first
`len(word)` characters of `s`. The word `s` itself counts as a prefix of `s`.
Entries that repeat are counted once per occurrence.

## Example 1

```
words  = ["s", "st", "stop", "top", "stone", "sto"]
s      = "stone"
output = 4
```

`"s"`, `"st"`, `"sto"` and `"stone"` are prefixes of `"stone"`.

## Example 2

```
words  = ["x", "x", "xy"]
s      = "xyz"
output = 3
```

Both copies of `"x"` count.

## Constraints

- `1 <= len(words) <= 1000`
- `1 <= len(words[i]), len(s) <= 10`
- All strings consist of lowercase English letters.
