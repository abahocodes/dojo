A string `s` lost all its spaces. You're given a dictionary `words`. Decide
whether `s` can be cut into a sequence of one or more pieces where **every
piece is a word from the dictionary**. Dictionary words may be used any number
of times, and not every word has to be used.

Return `true` if such a cut exists and `false` otherwise.

## Example 1

```
s      = "sunflowerseed"
words  = ["sun", "sunflow", "flower", "seed", "ers"]
output = true   # "sun" + "flower" + "seed"
```

## Example 2

```
s      = "pancakes"
words  = ["pan", "cake", "pancake"]
output = false  # the final "s" can never be covered
```

## Constraints

- `1 <= len(s) <= 300`
- `1 <= len(words) <= 1000`
- `1 <= len(words[i]) <= 20`
- `s` and every word contain only lowercase English letters
- All words are distinct
