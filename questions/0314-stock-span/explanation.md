# Approach: monotonic stack of blocking days

For day `i` we need the nearest earlier day with a strictly higher price;
call it `j` (or `-1`). The span is then `i - j`.

Keep a stack of candidate blocking days. When day `i` arrives, every day on
the stack whose price is `<= prices[i]` can never block a later day: any later
day that would reach past day `i` also reaches past them, and day `i` itself
blocks anything they would. So pop them. What remains on top is the nearest
strictly higher day. Push `i`. The prices on the stack are strictly
decreasing from bottom to top.

```python
def stock_span(prices):
    result = []
    stack = []
    for i, p in enumerate(prices):
        while stack and prices[stack[-1]] <= p:
            stack.pop()
        result.append(i - stack[-1] if stack else i + 1)
        stack.append(i)
    return result
```

## Complexity

- Time: O(n): each day is pushed and popped at most once.
- Space: O(n) for the stack.

## Pitfalls

- Popping only strictly smaller prices (`<`). Equal prices belong to the span,
  so `[5, 5, 5]` must give `[1, 2, 3]`.
- Forgetting to count the day itself, which makes every span one too short.
- Walking backwards from every day: O(n^2) on a rising price series of
  `10^5` days.
