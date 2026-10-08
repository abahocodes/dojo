# Approach: stack of runs

Scan `s` once and maintain a stack describing the characters that survive so
far, compressed into runs: each entry is a letter and how many times it repeats
consecutively at that point.

- If the incoming letter matches the letter on top of the stack, the run grows
  by one. When it reaches `k`, the whole run disappears, so pop it. The run
  underneath is now the top again, and the next letter may extend it — exactly
  the "pieces join together" behaviour of the cascade.
- Otherwise, push a new run of length 1.

When the scan ends, the stack holds the final string in order.

```python
def remove_k_duplicates(s, k):
    stack = []  # [letter, run length]
    for ch in s:
        if stack and stack[-1][0] == ch:
            stack[-1][1] += 1
            if stack[-1][1] == k:
                stack.pop()
        else:
            stack.append([ch, 1])
    return "".join(ch * count for ch, count in stack)
```

Because a run is removed the moment it reaches length `k`, no run on the stack
ever has length `>= k`, and two adjacent runs on the stack always hold
different letters — so nothing removable is left behind.

## Complexity

- Time: O(n) — each character is pushed or counted once, and each run is
  popped at most once.
- Space: O(n) for the stack.

## Pitfalls

- Storing single characters and re-counting the top `k` each time costs
  O(n * k).
- Forgetting the cascade: after a pop, the next letter must be compared with
  the *new* top, which the run stack handles automatically.
- Building the answer with repeated string concatenation in a loop can be
  quadratic in some languages; use a list / builder.
