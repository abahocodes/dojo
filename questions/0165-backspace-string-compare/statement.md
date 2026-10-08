Two strings `s` and `t` record keystrokes typed into an empty text box.
Lowercase letters are typed as-is; the character `#` is a **backspace**, which
erases the most recently typed character still in the box (and does nothing
if the box is empty).

Return `true` if typing `s` and typing `t` leave the same text in the box,
otherwise `false`.

Try to use only O(1) extra memory.

## Example 1

```
s      = "cab#t"
t      = "cat"
output = true    # "cab#t" types "ca", "cab", erases b, then "cat"
```

## Example 2

```
s      = "a#c"
t      = "b"
output = false   # s leaves "c", t leaves "b"
```

## Constraints

- `1 <= len(s), len(t) <= 200`
- `s` and `t` contain only lowercase English letters and `#`.
