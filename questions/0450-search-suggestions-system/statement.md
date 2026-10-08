A store has a catalogue of distinct product names, `products`. A customer types
`search_word` one character at a time. After each character, the store shows
up to **three** products whose names start with everything typed so far. When
more than three products match, it shows the three that come first in
lexicographic (dictionary) order, and it lists them in that order.

Return a list with one entry per typed character: entry `k` holds the
suggestions shown after the first `k + 1` characters of `search_word`. An entry
is `[]` when no product matches.

## Example 1

```
products    = ["monkey", "mouse", "monitor", "mop", "moneypot", "map"]
search_word = "mon"
output      = [["map", "moneypot", "monitor"],
               ["moneypot", "monitor", "monkey"],
               ["moneypot", "monitor", "monkey"]]
```

After `"m"` all six names match, and the three smallest are shown. After
`"mo"` the name `"map"` drops out.

## Example 2

```
products    = ["lamp"]
search_word = "lamps"
output      = [["lamp"], ["lamp"], ["lamp"], ["lamp"], []]
```

## Constraints

- `1 <= len(products) <= 1000`
- `1 <= len(products[i])` and the names total at most `2 * 10^4` characters.
- All product names are distinct.
- `1 <= len(search_word) <= 1000`
- All strings consist of lowercase English letters.
