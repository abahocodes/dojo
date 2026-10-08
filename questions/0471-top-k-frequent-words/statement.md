Given a list of lowercase `words` and an integer `k`, return the `k` distinct
words that occur most often.

Order the result by number of occurrences, most frequent first. When two words
occur the same number of times, the one that comes first alphabetically
(lexicographically, comparing character by character) goes first. The order
of the whole result is checked, not just which words appear in it.

## Example 1

```
words  = ["pear", "fig", "pear", "kiwi", "fig", "pear", "apple"]
k      = 2
output = ["pear", "fig"]
```

`pear` appears three times and `fig` twice.

## Example 2

```
words  = ["dog", "cat", "ant", "cat", "dog", "bee"]
k      = 3
output = ["cat", "dog", "ant"]
```

`cat` and `dog` both appear twice, so `cat` comes first. `ant` and `bee` both
appear once; `ant` wins the last spot.

## Constraints

- `1 <= len(words) <= 10^4`
- `1 <= len(words[i]) <= 10`, lowercase English letters only.
- `1 <= k <=` the number of distinct words.
