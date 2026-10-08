Given a lowercase string `s`, return the length of the longest substring in
which each of the five vowels `'a'`, `'e'`, `'i'`, `'o'` and `'u'` occurs an
even number of times. A vowel that does not occur at all has count zero,
which is even. Consonants may occur any number of times.

If no non-empty substring qualifies, return `0`.

## Example 1

```
s      = "bookkeeper"
output = 8   # "bookkeep": two o's and two e's
```

## Example 2

```
s      = "banana"
output = 5   # "banan": two a's; all of "banana" has three
```

## Constraints

- `1 <= len(s) <= 5 * 10^5`
- `s` consists of lowercase English letters.
