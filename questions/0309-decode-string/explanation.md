# Approach: stack of (prefix, count)

Keep a buffer for the text of the innermost group being read and a number
being accumulated. On `[` the current buffer and the count are pushed and a
new, empty buffer starts for the body. On `]` the body is complete: pop the
prefix and count it belongs to, and continue with `prefix + body * count`.
After the last character the buffer is the answer.

```python
def decode_string(s):
    stack = []
    buf = []
    k = 0
    for ch in s:
        if ch.isdigit():
            k = k * 10 + int(ch)
        elif ch == "[":
            stack.append((buf, k))
            buf, k = [], 0
        elif ch == "]":
            prev, times = stack.pop()
            prev.append("".join(buf) * times)
            buf = prev
        else:
            buf.append(ch)
    return "".join(buf)
```

A recursive descent parser that returns at each `]` is equally valid; the
explicit stack just avoids recursion.

## Complexity

- Time: O(len(s) + L * d) in the worst case, where `L` is the output length
  and `d` the nesting depth, because each level copies its expanded body once
  more. With `len(s) <= 100` and `L <= 10^5` this is tiny.
- Space: O(L) for the buffers on the stack.

## Pitfalls

- Reading only one digit for the count: `"12[a]"` needs 12 copies.
- Forgetting to reset the count after pushing it, so the next group's count
  starts from the old value.
- Appending a nested group's expansion to the outer result instead of to the
  enclosing body, which loses the outer repetition.
