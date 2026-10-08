You are given a list of lowercase `words`. A word is **buildable** if you can
reach it by starting from a single letter and adding one letter at a time to
the end, with every intermediate string also present in `words`. For example,
`"top"` is buildable only if `"t"`, `"to"` and `"top"` all appear in the list.

Return the longest buildable word. If several buildable words share that
length, return the one that comes first alphabetically. If no word is
buildable, return the empty string `""`.

## Example 1

```
words  = ["t", "to", "top", "tops", "tap", "ta"]
output = "tops"     # t -> to -> top -> tops
```

## Example 2

```
words  = ["b", "ba", "bat", "c", "ca", "cab", "ban"]
output = "ban"      # "ban", "bat" and "cab" are all buildable; "ban" is first alphabetically
```

## Constraints

- `1 <= len(words) <= 1000`
- `1 <= len(words[i]) <= 30`
- Every word contains only lowercase English letters.
