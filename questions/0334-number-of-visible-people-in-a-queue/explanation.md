# Approach: right-to-left monotonic stack

Looking right from person `i`, the visible people are the "running maxima"
of the line after `i`, up to and including the first person taller than `i`.
Everyone else is hidden behind a taller person who is still shorter than `i`
or is blocked by the first taller person.

Scan from right to left with a stack whose heights decrease from bottom to
top. The stack holds exactly the running maxima seen from the current
position. For person `i`:

- Pop every person shorter than `heights[i]`. Each is a running maximum
  before anyone taller than `i`, so `i` sees them. They can be discarded:
  for anyone further left, `i` stands in front of them and is taller.
- If the stack is not empty afterwards, its top is the first person taller
  than `i`, and `i` sees that person too (but nobody behind them).
- Push `i`.

```python
def can_see_persons_count(heights):
    answer = [0] * len(heights)
    stack = []
    for i in range(len(heights) - 1, -1, -1):
        seen = 0
        while stack and stack[-1] < heights[i]:
            stack.pop()
            seen += 1
        if stack:
            seen += 1
        answer[i] = seen
        stack.append(heights[i])
    return answer
```

## Complexity

- Time: O(n). Each person is pushed and popped at most once.
- Space: O(n) for the stack and the answer.

## Pitfalls

- Counting only the people popped and forgetting the first taller person,
  who is visible but stays on the stack.
- Counting everybody to the right who is shorter than `i`: people hidden
  behind an intermediate taller-but-still-shorter-than-`i` person do not count.
- Scanning right from every person: a line sorted in decreasing order makes
  every scan run to the end, O(n^2) for `10^5` people.
