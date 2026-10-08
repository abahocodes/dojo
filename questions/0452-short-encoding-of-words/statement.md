You want to pack a list of lowercase `words` into a single **reference string**
`s`. The string is made of chunks, and every chunk is terminated by a `#`
character. A word counts as stored if it can be read off `s` by starting at
some position and stopping exactly at a `#`, that is, if it is a **suffix of
one of the chunks**.

Return the length (including the `#` characters) of the shortest reference
string that stores every word in the list. A word that is a suffix of another
word can share that word's chunk instead of needing its own.

## Example 1

```
words  = ["bell", "ell", "bat"]
output = 9       # s = "bell#bat#"; "ell" is read from inside "bell#"
```

## Example 2

```
words  = ["go"]
output = 3       # s = "go#"
```

## Constraints

- `1 <= len(words) <= 2000`
- `1 <= len(words[i]) <= 7`
- Every word consists of lowercase English letters. Words may repeat.
