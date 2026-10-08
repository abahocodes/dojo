You are given a list of distinct lowercase `words`, and no word in the list is
a prefix of another word in the list. For every word, find the shortest prefix
of it that is **not** a prefix of any other word in the list. Such a prefix
always exists, because the whole word qualifies.

Return these prefixes as a list, in the same order as the input words.

## Example 1

```
words  = ["zebra", "dog", "duck", "dove"]
output = ["z", "dog", "du", "dov"]
```

`"d"` and `"do"` are shared by several words, so `"dog"` needs all three
letters, while `"z"` alone already identifies `"zebra"`.

## Example 2

```
words  = ["apple"]
output = ["a"]
```

## Constraints

- `1 <= len(words) <= 10^4`
- `1 <= len(words[i])` and the total number of characters is at most `10^5`
- Words consist of lowercase English letters, are distinct, and none is a
  prefix of another.
