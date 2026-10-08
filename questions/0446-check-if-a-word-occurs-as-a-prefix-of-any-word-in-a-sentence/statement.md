A `sentence` is made of lowercase words separated by single spaces, with no
leading or trailing spaces. Given a string `search_word`, find the **first**
word of the sentence that begins with `search_word`, and return its position
counting from `1`. If no word begins with `search_word`, return `-1`.

A word begins with `search_word` when its first `len(search_word)` characters
equal `search_word`. A word equal to `search_word` counts.

## Example 1

```
sentence    = "the quick fox quietly quits"
search_word = "qui"
output      = 2
```

Words 2, 4 and 5 start with `"qui"`. The earliest one is `"quick"`.

## Example 2

```
sentence    = "nobody here knows"
search_word = "now"
output      = -1
```

`"knows"` contains `"now"`, but does not start with it.

## Constraints

- `1 <= len(sentence) <= 100`
- `1 <= len(search_word) <= 10`
- `sentence` holds lowercase letters and single spaces between words.
- `search_word` holds lowercase letters.
