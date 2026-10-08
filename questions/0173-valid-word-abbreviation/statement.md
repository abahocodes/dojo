A word can be **abbreviated** by choosing some substrings of it and replacing
each chosen substring by its length, written in decimal with no leading zeros.
The chosen substrings must be non-empty and must not touch each other (two
replaced pieces are always separated by at least one kept letter), so every
maximal group of digits in an abbreviation stands for exactly one replaced
piece.

For example, `"s4a2"` means the letter `s`, four skipped letters, the letter
`a`, then two skipped letters. It abbreviates any 8-letter word of the shape
`s????a??`, such as `"standard"`.

Given a lowercase `word` and a string `abbr` of lowercase letters and digits,
return `true` if `abbr` is a valid abbreviation of `word`, otherwise `false`.
A number that starts with `0` (including the number `0` itself) is never valid.

## Example 1

```
word   = "localization"
abbr   = "l10n"
output = true     # 'l', then 10 skipped letters, then 'n'
```

## Example 2

```
word   = "apple"
abbr   = "a03e"
output = false    # "03" has a leading zero
```

## Constraints

- `1 <= len(word) <= 20`, `word` consists of lowercase English letters.
- `1 <= len(abbr) <= 10`, `abbr` consists of lowercase English letters and
  digits.
- Every number written in `abbr` fits in a 32-bit signed integer.
