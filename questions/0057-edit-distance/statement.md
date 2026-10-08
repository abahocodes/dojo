You may change a string one character at a time using three kinds of edit:

- **insert** a character at any position,
- **delete** any character,
- **replace** any character with a different one.

Each edit costs `1`. Return the **minimum number of edits** that turn `word1`
into `word2`.

## Example 1

```
word1  = "kitten"
word2  = "sitting"
output = 3      # k->s, e->i, then insert g at the end
```

## Example 2

```
word1  = ""
word2  = "abc"
output = 3      # insert a, b and c
```

## Constraints

- `0 <= len(word1), len(word2) <= 500`
- Both strings contain only lowercase English letters
