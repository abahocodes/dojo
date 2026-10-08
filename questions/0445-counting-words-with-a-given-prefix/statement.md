You are given a list of strings `words` and a string `pref`. Count how many
entries of `words` begin with `pref`, meaning their first `len(pref)`
characters are exactly `pref`. Duplicate entries count separately, and a word
equal to `pref` counts too.

## Example 1

```
words  = ["parcel", "party", "apart", "part", "pa"]
pref   = "par"
output = 3
```

`"parcel"`, `"party"` and `"part"` start with `"par"`. `"apart"` only contains
it, and `"pa"` is too short.

## Example 2

```
words  = ["zebra", "zone", "maze"]
pref   = "ze"
output = 1
```

## Constraints

- `1 <= len(words) <= 1000`
- `1 <= len(words[i]), len(pref) <= 100`
- All strings consist of lowercase English letters.
