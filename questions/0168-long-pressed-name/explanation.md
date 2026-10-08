# Approach: two pointers

Every character of `typed` is either the next character of `name` or a repeat
of the character just typed. Scan `typed` once with a pointer `i` into
`name`:

- if `typed[j] == name[i]`, it is the next real keystroke: advance `i`;
- else if `typed[j] == typed[j - 1]`, it is a stuck key: skip it;
- else it can't be explained: return `false`.

Matching greedily is safe: when `typed[j]` equals both `name[i]` and the
previous character, consuming `name[i]` never hurts, because a later repeat
can still be absorbed as a long press.

At the end every character of `name` must have been consumed.

```python
def is_long_pressed_name(name: str, typed: str) -> bool:
    i = 0
    for j, c in enumerate(typed):
        if i < len(name) and name[i] == c:
            i += 1
        elif j == 0 or typed[j - 1] != c:
            return False
    return i == len(name)
```

## Complexity

- Time: O(len(name) + len(typed)).
- Space: O(1).

## Pitfalls

- Returning `true` without checking `i == len(name)`: `"abc"` / `"ab"` would
  pass.
- Allowing extra characters at the end of `typed` that don't repeat the last
  one: `"abc"` / `"abcd"` is `false`.
- Letting one long run cover two runs of `name`: `"aab"` / `"abb"` is `false`.
