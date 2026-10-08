You are given a dictionary `words` and a string `stream`. The characters of
`stream` arrive one at a time, from left to right.

After each arrival, decide whether the text received so far **ends with** at
least one word of the dictionary — that is, whether some word in `words` is a
suffix of the characters seen up to and including the newest one.

Return a list of booleans with one entry per character of `stream`, in arrival
order: entry `i` is `true` when some word is a suffix of `stream[0..i]`.

## Example 1

```
words  = ["ab", "dab", "c"]
stream = "xdabc"
output = [false, false, false, true, true]
```

After `"xda"` nothing matches. After `"xdab"`, both `"ab"` and `"dab"` are
suffixes. After `"xdabc"`, `"c"` is a suffix.

## Example 2

```
words  = ["aaa", "ba"]
stream = "aaaba"
output = [false, false, true, false, true]
```

## Constraints

- `1 <= len(words) <= 2000`
- `1 <= len(words[i]) <= 200`
- `1 <= len(stream) <= 4 * 10^4`
- `words[i]` and `stream` consist of lowercase English letters.
