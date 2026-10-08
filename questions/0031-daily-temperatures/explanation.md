# Approach: monotonic stack of unresolved days

Process days in order, keeping a stack of indices still waiting for a warmer
day. Their temperatures are non-increasing from bottom to top, because a warmer
day would already have resolved everything colder below it.

When day `i` arrives, every stacked day that is strictly colder is resolved by
day `i`: pop it and store the distance. Then push `i`.

```python
def daily_temperatures(temperatures):
    answer = [0] * len(temperatures)
    stack = []
    for i, temp in enumerate(temperatures):
        while stack and temperatures[stack[-1]] < temp:
            j = stack.pop()
            answer[j] = i - j
        stack.append(i)
    return answer
```

## Complexity

- Time: O(n) — each index is pushed once and popped at most once.
- Space: O(n) for the stack in the worst case (a steadily cooling stretch).

## Pitfalls

- Use a strict `<`: a day with the same reading does not count as warmer.
- Push indices, not temperatures, so you can compute the distance.
- Days never popped already hold `0`; there is nothing to clean up afterwards.
