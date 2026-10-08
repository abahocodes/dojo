# Approach: greedy monotonic stack

Scan `s` once and maintain the best answer for the prefix seen so far as a
stack. Two facts drive the greedy:

1. If the current letter `c` is already in the stack, skip it. The copy in
   the stack is placed at an earlier position, and it was kept because
   everything after it was not worth swapping it out for.
2. Otherwise, while the top of the stack is larger than `c` **and** that top
   letter appears again later in `s`, pop it. Placing `c` before it gives a
   smaller string, and we can re-add the popped letter later.

A precomputed `last[letter]` index answers "appears again later" in O(1).

```python
def remove_duplicate_letters(s):
    last = {c: i for i, c in enumerate(s)}
    stack, used = [], set()
    for i, c in enumerate(s):
        if c in used:
            continue
        while stack and stack[-1] > c and last[stack[-1]] > i:
            used.discard(stack.pop())
        stack.append(c)
        used.add(c)
    return "".join(stack)
```

## Complexity

- Time: O(n): each character is pushed and popped at most once.
- Space: O(1) besides the output: the stack holds at most 26 letters.

## Pitfalls

- Popping a letter that never appears again: it would then be missing from
  the answer. Always check `last[top] > i`.
- Not skipping letters already in the stack, which produces duplicates or
  pops a letter in favour of another copy of the same letter.
- Forgetting to clear the "used" flag of a popped letter, so its later copy
  is wrongly skipped.
- Sorting the distinct letters: the answer must be a subsequence of `s`, so
  `"cba"` has to stay `"cba"`.
