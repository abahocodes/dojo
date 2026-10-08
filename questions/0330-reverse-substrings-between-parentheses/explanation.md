# Approach: jump between matching parentheses

Pair up the parentheses first: push the index of every `"("` on a stack, and
when a `")"` arrives, pop its partner and record the match in both directions.

Now read the string with a cursor and a direction (`+1` = rightwards). Letters
are emitted as the cursor passes over them. When the cursor lands on a
parenthesis, it teleports to the matching one and reverses direction. Entering
a pair from the left therefore reads its contents right-to-left — the reversal —
and nested pairs flip the direction again, exactly undoing one level of
reversal each.

```python
def reverse_parentheses(s):
    n = len(s)
    partner = [0] * n
    opens = []
    for i, ch in enumerate(s):
        if ch == "(":
            opens.append(i)
        elif ch == ")":
            j = opens.pop()
            partner[i], partner[j] = j, i
    out = []
    i, step = 0, 1
    while i < n:
        if s[i] in "()":
            i = partner[i]
            step = -step
        else:
            out.append(s[i])
        i += step
    return "".join(out)
```

The cursor always leaves the string at the right end: the outermost level is
read rightwards, and every pair it enters is eventually exited through the
opposite parenthesis.

## Complexity

- Time: O(n) — each character is visited exactly twice at most (once per
  pass), and every letter is emitted once.
- Space: O(n) for the partner table and output.

## Pitfalls

- With the stack-of-strings method, reversing at every `")"` costs O(n) each
  time, giving O(n^2) for deeply nested input.
- Forgetting that the parentheses themselves must not appear in the output.
- Handling an empty pair `"()"`: the cursor jumps to `")"`, flips, steps back
  onto `"("`, jumps to `")"` again and flips once more — it continues rightwards
  correctly with no output.
