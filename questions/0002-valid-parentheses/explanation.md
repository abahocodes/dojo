# Approach: stack of open brackets

Openers wait on a stack until they are closed. A closer must match whatever
opener is on top — anything else means the nesting is crossed or the closer
has no partner. When the scan finishes, any leftover openers were never closed.

```python
def is_valid(s):
    pairs = {")": "(", "]": "[", "}": "{"}
    stack = []
    for ch in s:
        if ch in pairs:
            if not stack or stack[-1] != pairs[ch]:
                return False
            stack.pop()
        else:
            stack.append(ch)
    return not stack
```

## Complexity

- Time: O(n) — one pass, O(1) work per character.
- Space: O(n) in the worst case (all openers).

## Pitfalls

- Forgetting the final `not stack` check: `"(("` never fails during the scan.
- Popping from an empty stack on a leading closer like `")("`.
- A simple counter per bracket type is not enough: `"([)]"` has balanced counts
  but crossed nesting.
- Early exit: an odd-length string can never be balanced.
