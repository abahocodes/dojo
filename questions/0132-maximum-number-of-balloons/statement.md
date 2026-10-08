You have a pile of lowercase letter tiles, given as the string `text`. Each
tile can be used at most once. Using these tiles, you want to spell the word
`"balloon"` as many separate times as possible.

Return the largest number of complete copies of `"balloon"` you can spell.
Leftover tiles are fine; the order of letters in `text` does not matter.

## Example 1

```
text   = "loonbalxballpoon"
output = 2    # two of every b, a, n and four each of l and o
```

## Example 2

```
text   = "balon"
output = 0    # "balloon" needs two 'l' and two 'o'
```

## Constraints

- `1 <= len(text) <= 10^4`
- `text` consists only of lowercase English letters.
