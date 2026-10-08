# Approach: balance counter

Match greedily from left to right. Keep `open`, the number of `(` not yet
matched.

- On `(`, increment `open`.
- On `)`, if `open > 0` it closes one of them; otherwise no `(` before it is
  available, so we must insert a `(` for it (`added += 1`).

At the end, each unmatched `(` needs one inserted `)`. The total is
`added + open`.

This is optimal: an unmatched `)` has more `)` than `(` in the prefix ending
at it, and only an inserted `(` fixes that; similarly each leftover `(`
needs a `)` after it. One insertion fixes exactly one unmatched character, so
no fewer insertions can work.

```python
def min_add_to_make_valid(s):
    open_ = added = 0
    for c in s:
        if c == "(":
            open_ += 1
        elif open_:
            open_ -= 1
        else:
            added += 1
    return added + open_
```

## Complexity

- Time: O(n), one pass.
- Space: O(1).

## Pitfalls

- Returning `abs(count("(") - count(")"))`: for `")("` the counts are equal,
  but two insertions are needed.
- Letting the counter go negative and "paying back" later. A `)` that comes
  before any `(` can never be matched by a later `(`.
- Forgetting the leftover `open` count at the end.
