You are playing a word game. Starting from `begin_word`, you may change exactly
one letter at a time, and every word you produce along the way must appear in
`word_list`. The goal is to reach `end_word`.

A **ladder** is the full sequence of words from `begin_word` to `end_word`,
counting both ends. `begin_word` itself does not need to be in `word_list`,
but `end_word` does.

Return the number of words in the shortest ladder, or `0` if no ladder exists.

## Example 1

```
begin_word = "cold"
end_word   = "warm"
word_list  = ["cord", "card", "ward", "warm", "worm", "word", "wore"]
output     = 5      # cold -> cord -> card -> ward -> warm
```

## Example 2

```
begin_word = "hit"
end_word   = "cog"
word_list  = ["hot", "dot", "dog", "lot", "log"]
output     = 0      # "cog" is not in the list, so it can't be reached
```

## Constraints

- `1 <= len(begin_word) <= 10`
- `end_word` and every word in `word_list` have the same length as `begin_word`.
- `1 <= len(word_list) <= 5000`
- All words use lowercase English letters only.
- `begin_word != end_word`
- The words in `word_list` are distinct.
