# Approach: split and test each word in order

Split the sentence into words, walk them from left to right, and return the
1-based position of the first word that starts with `search_word`. Returning
on the first hit guarantees the earliest position.

```python
def is_prefix_of_word(sentence, search_word):
    for position, word in enumerate(sentence.split(" "), start=1):
        if word.startswith(search_word):
            return position
    return -1
```

The same idea works without building a list of words: scan the sentence with
a pointer, treat every character after a space (and index 0) as the start of a
word, and compare `search_word` there, character by character.

## Complexity

- Time: O(len(sentence)) for the split, plus at most `len(search_word)`
  character comparisons per word.
- Space: O(len(sentence)) for the split words (O(1) with the pointer scan).

## Pitfalls

- Positions are 1-based. Returning the 0-based index is the most common slip.
- `search_word in sentence` or `search_word in word` also matches the middle of
  a word (`"now"` inside `"knows"`).
- Return the first match, not the last: break out as soon as one is found.
- A `search_word` longer than the word cannot match. Hand-written comparisons
  must not read past the end of the word.
