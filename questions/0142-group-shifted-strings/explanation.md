# Approach: a shift-invariant key

Shift any string until its first letter is `'a'`. Every string of a family
lands on the same normalized string, and strings from different families
land on different ones, so the normalized string is a perfect grouping key.
Normalizing means replacing each letter `c` by `(c - s[0]) mod 26`.

Group the strings in a hash map from key to the list of strings with that
key. To keep the output deterministic, the reference solutions also remember
the order in which keys first appear (Python dicts already do).

```python
def group_strings(strings):
    groups = {}
    for s in strings:
        key = "".join(chr((ord(c) - ord(s[0])) % 26 + ord("a")) for c in s)
        groups.setdefault(key, []).append(s)
    return list(groups.values())
```

## Complexity

- Time: O(L), where L is the total length of all strings.
- Space: O(L) for the keys and groups.

## Pitfalls

- Computing `c - s[0]` without the modulo (or with a negative modulo in Java,
  C++, Go and JavaScript, where `%` keeps the sign): `"ba"` and `"az"` are in
  the same family only once `-1` wraps to `25`. Add 26 before taking `% 26`.
- Building the key from differences without a separator when they are
  printed as numbers: `[1, 12]` and `[11, 2]` must not collide. Mapping each
  difference back to a single letter avoids this.
- Dropping duplicates: every copy of a string belongs in the output.
