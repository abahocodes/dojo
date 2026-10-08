You are given a list of lowercase strings `words`. Two words belong together if
one is a rearrangement of the other's letters (they use exactly the same
letters, the same number of times).

Split `words` into groups so that every group holds all the words that are
rearrangements of each other, and no two groups could be merged. Every entry of
`words` goes into exactly one group; if the same word appears several times, it
appears that many times in its group.

Return the list of groups. The groups may be listed in any order, and the
words inside each group may be listed in any order.

## Example 1

```
words  = ["pots", "tops", "cat", "stop", "act", "dog"]
output = [["pots", "tops", "stop"], ["cat", "act"], ["dog"]]
```

## Example 2

```
words  = ["", "b", ""]
output = [["", ""], ["b"]]      # the empty string is a word too
```

## Constraints

- `1 <= len(words) <= 10^4`
- `0 <= len(words[i]) <= 100`
- `words[i]` contains only lowercase English letters `a`-`z`.
