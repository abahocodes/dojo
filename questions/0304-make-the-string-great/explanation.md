# Approach: a stack of surviving characters

Scan the string and keep the survivors on a stack. When the next character
clashes with the top, meaning the same letter in the opposite case, both
vanish: pop. Otherwise push. A pop exposes an older survivor, so the chain
reaction in `"aBbA"` happens on its own.

```python
def make_good(s):
    stack = []
    for ch in s:
        if stack and stack[-1] != ch and stack[-1].lower() == ch.lower():
            stack.pop()
        else:
            stack.append(ch)
    return "".join(stack)
```

In ASCII a lowercase letter's code is its uppercase code plus 32, so the
clash test can also be `abs(a - b) == 32` (or `a ^ b == 32`), which is what
the Java, C++ and Go solutions use.

## Complexity

- Time: O(n), each character is pushed and popped at most once.
- Space: O(n) for the stack.

## Pitfalls

- `"aa"` is not a clash. The letters must differ in case, not just match
  ignoring case.
- A single left-to-right pass that does not look back misses chains such as
  `"aBbA"`. Look at the survivor, not the original neighbour.
- `abs(a - b) == 32` is only correct because the input is letters. For
  arbitrary characters, such as `'@'` and `'`'`, it gives false positives.
