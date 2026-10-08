# Approach: depth counter

A stack of open parentheses would work, but only its size matters, so a single
depth counter replaces it. A character is the outer opening of a block when it
is a `(` seen at depth 0, and the outer closing when it is a `)` that brings
the depth back to 0. Every other character is copied to the output.

```python
def remove_outer_parentheses(s):
    out = []
    depth = 0
    for ch in s:
        if ch == "(":
            if depth > 0:
                out.append(ch)
            depth += 1
        else:
            depth -= 1
            if depth > 0:
                out.append(ch)
    return "".join(out)
```

## Complexity

- Time: O(n), one pass.
- Space: O(n) for the output (O(1) besides it).

## Pitfalls

- Order of the update and the test: for `(` test before incrementing, for `)`
  decrement before testing. Mixing them keeps or drops the wrong characters.
- Removing only the first and last character of the whole string. Every block
  loses its own outer pair.
- Building the result by repeated string concatenation in a loop, which is
  quadratic in some languages. Use a builder or list.
