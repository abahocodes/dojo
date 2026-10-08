# Approach: count the magazine, spend on the note

The note can be built exactly when, for every letter, the note needs no more
copies than the magazine has. Count the magazine's letters in an array of 26,
then decrement for each letter of the note and fail as soon as a count goes
negative.

```python
def can_construct(ransom_note, magazine):
    if len(ransom_note) > len(magazine):
        return False
    counts = [0] * 26
    for ch in magazine:
        counts[ord(ch) - 97] += 1
    for ch in ransom_note:
        i = ord(ch) - 97
        counts[i] -= 1
        if counts[i] < 0:
            return False
    return True
```

The length check up front is an optional shortcut: a longer note can never
fit.

## Complexity

- Time: O(n + m) for the two strings.
- Space: O(1): 26 counters.

## Pitfalls

- Checking only that every note letter *appears* in the magazine, ignoring
  how many times (`"hello"` / `"hole"`).
- Removing letters from a list or string one at a time, which makes the
  solution O(n * m).
- Counting the note and the magazine the wrong way round.
