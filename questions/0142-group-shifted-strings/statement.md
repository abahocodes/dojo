Shifting a string advances every one of its letters to the next letter of
the alphabet, with `'z'` wrapping around to `'a'`. For example, shifting
`"azy"` gives `"baz"`, and shifting it again gives `"cba"`.

Two strings belong to the same **shifting family** if one can be turned into
the other by shifting it some number of times. Given a list of lowercase
strings, group them by shifting family.

Return the groups in any order; the strings inside a group may also be in
any order. If a string appears several times in the input, every copy goes
into the same group.

## Example 1

```
strings = ["ace", "bdf", "yac", "mo", "xz", "q", "k", "hello"]
output  = [["ace", "bdf", "yac"], ["mo", "xz"], ["q", "k"], ["hello"]]
```

## Example 2

```
strings = ["zz", "aa", "zz", "ab"]
output  = [["zz", "aa", "zz"], ["ab"]]
```

## Constraints

- `1 <= len(strings) <= 200`
- `1 <= len(strings[i]) <= 50`
- `strings[i]` consists of lowercase English letters.
