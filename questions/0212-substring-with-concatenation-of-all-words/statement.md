You are given a string `s` and a list `words` of strings that all have the
**same length**. The list may contain the same word more than once.

A window of `s` is a **perfect concatenation** if it can be cut into
consecutive pieces, one per entry of `words`, so that every entry of `words`
(counting repeats) is used exactly once, in any order. Such a window always
has length `len(words) * len(words[0])`.

Return the start index of every perfect-concatenation window of `s`, in
increasing order. Return an empty list if there are none. Windows may
overlap.

## Example 1

```
s      = "xyzabcabcxyz"
words  = ["abc", "xyz"]
output = [0, 6]    # "xyz" + "abc" at 0, "abc" + "xyz" at 6
```

## Example 2

```
s      = "dogcatcatdog"
words  = ["cat", "dog", "cat"]
output = [0, 3]    # "dogcatcat" and "catcatdog"; "cat" is needed twice
```

## Constraints

- `1 <= len(s) <= 10^4`
- `1 <= len(words) <= 5000`
- `1 <= len(words[i]) <= 30`, and all words have the same length
- `s` and the words consist of lowercase English letters
