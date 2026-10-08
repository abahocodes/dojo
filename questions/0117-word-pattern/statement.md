You are given a `pattern` of lowercase letters and a string `s` of lowercase
words separated by single spaces. Return `true` if `s` **follows** the
pattern, meaning:

- `s` has exactly as many words as `pattern` has letters, and
- there is a one-to-one correspondence between letters and words: the `i`-th
  letter pairs with the `i`-th word, equal letters always pair with equal
  words, and different letters always pair with different words.

Otherwise return `false`.

## Example 1

```
pattern = "xyyx"
s       = "sun moon moon sun"
output  = true    # x <-> sun, y <-> moon
```

## Example 2

```
pattern = "aba"
s       = "red red red"
output  = false   # 'a' and 'b' would both pair with "red"
```

## Constraints

- `1 <= len(pattern) <= 300`
- `1 <= len(s) <= 3000`
- `pattern` contains only lowercase English letters.
- `s` contains only lowercase English letters and single spaces, with no
  leading or trailing space.
