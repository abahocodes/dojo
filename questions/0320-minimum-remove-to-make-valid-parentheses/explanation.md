# Approach: stack of open indices

Pairing every `)` with the nearest unpaired `(` before it is the classic
stack discipline:

- on `(`, push its index;
- on `)`, pop if the stack is non-empty (a pair is formed), otherwise this
  `)` can never be matched and is deleted;
- letters are kept untouched.

When the scan ends, the indices still on the stack are the unpaired `(`, and
they are deleted too. The number of deletions is minimal: an unpaired `)`
has more `)` than `(` before it in its prefix, and the leftover `(` are
exactly the surplus, so every valid result needs at least as many deletions.

```python
def min_remove_to_make_valid(s):
    chars = list(s)
    stack = []
    for i, c in enumerate(chars):
        if c == "(":
            stack.append(i)
        elif c == ")":
            if stack:
                stack.pop()
            else:
                chars[i] = ""
    for i in stack:
        chars[i] = ""
    return "".join(chars)
```

## Complexity

- Time: O(n), two linear passes.
- Space: O(n) for the stack and the output.

## Pitfalls

- Removing the *last* surplus `(` characters instead of the unpaired ones.
  It removes the same number of characters but can give a different string:
  for `"(a()"` the rule keeps `"a()"`, not `"(a)"`.
- Building the result with repeated string concatenation or deletion inside
  the loop, which is quadratic in some languages.
- Forgetting the second cleanup step for leftover `(`.
