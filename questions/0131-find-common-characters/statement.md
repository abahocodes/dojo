Given an array of lowercase words, find the letters that every word
contains, **with multiplicity**: if a letter occurs at least `m` times in
every word, it appears `m` times in the answer (with `m` the largest such
number).

Return the letters as one-character strings, sorted in **alphabetical
order**. Return an empty list if no letter is shared by all words.

## Example 1

```
words  = ["banana", "bandana", "cabana"]
output = ["a", "a", "a", "b", "n"]
```

## Example 2

```
words  = ["moon", "noon", "onto"]
output = ["n", "o", "o"]
```

## Constraints

- `1 <= len(words) <= 100`
- `1 <= len(words[i]) <= 100`
- Words consist of lowercase English letters.
