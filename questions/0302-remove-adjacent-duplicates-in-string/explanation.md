# Approach: a stack of surviving characters

Scan left to right and keep the characters that have survived so far on a
stack. When the next character equals the top of the stack, the two are
neighbours in the current string, so both disappear: pop the top. Otherwise
push the new character. A pop exposes an older character, which is exactly
the chain reaction in `"xyyxz"`.

```python
def remove_duplicates(s):
    stack = []
    for ch in s:
        if stack and stack[-1] == ch:
            stack.pop()
        else:
            stack.append(ch)
    return "".join(stack)
```

A `StringBuilder` (Java), `std::string` (C++) or byte slice (Go) works as the
stack and turns into the result directly.

## Complexity

- Time: O(n), each character is pushed and popped at most once.
- Space: O(n) for the stack.

## Pitfalls

- Restarting a scan from the beginning after each removal is O(n^2) and too
  slow for `10^5` characters.
- A run of three equal letters such as `"aaa"` leaves one `"a"`. Only pairs
  disappear.
- Building the result by repeated string concatenation, or by deleting from
  the middle of a string, can be quadratic. Use a stack or builder.
