Decide whether the text `s` is a palindrome when you look only at its letters
and digits. Ignore every other character (spaces, punctuation, symbols) and
treat uppercase and lowercase letters as equal. Then check whether the
remaining sequence reads the same forwards and backwards.

A text with no letters or digits at all counts as a palindrome.

## Example 1

```
s      = "Was it a car, or a cat I saw?"
output = true     # "wasitacaroracatisaw" reads the same both ways
```

## Example 2

```
s      = "race a car"
output = false    # "raceacar" reversed is "racaecar"
```

## Constraints

- `1 <= len(s) <= 20000`
- `s` contains only printable ASCII characters.
