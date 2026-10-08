# Approach: two maps, one per direction

The substitution must be a function (each character of `s` has one image) and
injective (no two characters of `s` share an image). Scan the strings in
parallel and keep two maps: `forward` from characters of `s` to characters of
`t`, and `backward` the other way. For each pair `(a, b)`:

- if `a` is already mapped, it must map to `b`;
- if `b` is already the image of something, that something must be `a`.

Any violation means no valid substitution exists. If the scan finishes, the
pairs seen define a valid one.

```python
def is_isomorphic(s, t):
    forward = {}
    backward = {}
    for a, b in zip(s, t):
        if forward.setdefault(a, b) != b or backward.setdefault(b, a) != a:
            return False
    return True
```

Since the alphabet is printable ASCII, the maps can be arrays of 128 entries
indexed by character code, which is what the Java, C++ and Go solutions do.

## Complexity

- Time: O(n), one pass over the strings.
- Space: O(1): each map holds at most 95 entries.

## Pitfalls

- Checking only `s -> t`. `"ab"` / `"cc"` passes that check but is `false`.
- Checking only `t -> s`, which fails the mirror case `"cc"` / `"ab"`.
- Comparing character frequencies: `"abab"` and `"aabb"` have the same counts
  but are not isomorphic. The positions matter.
- Treating the space as a separator. It is just another character.
